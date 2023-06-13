use cpp::cpp;

cpp! {{
    #include <QtGui/QKeyEvent>
    #include <QtGui/QGuiApplication>
    #include <QtGui/QWindow>
}}

pub fn send_key_press(key_code: i32, is_auto_repeat: bool) {
    cpp!(unsafe [key_code as "int", is_auto_repeat as "bool"] {
        QWindow* window = QGuiApplication::focusWindow();
        if (window != nullptr)
        {
            QKeyEvent evt(QEvent::KeyPress, key_code, Qt::NoModifier, QString(), is_auto_repeat);
            QGuiApplication::sendEvent(window, &evt);
        }
    });
}

pub fn send_key_release(key_code: i32) {
    cpp!(unsafe [key_code as "int"] {
        QWindow* window = QGuiApplication::focusWindow();
        if (window != nullptr)
        {
            QKeyEvent evt(QEvent::KeyRelease, key_code, Qt::NoModifier);
            QGuiApplication::sendEvent(window, &evt);
        }
    });
}
