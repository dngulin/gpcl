import QtQml
import QtQuick
import QtQuick.Layouts
import QtQuick.Effects
import GpcLauncherTypes

Window {
    id: window
    visible: true
    visibility: Window.FullScreen
    title: "Gamepad Controlled Launcher"

    LauncherApp {
        id: app
    }

    Component.onCompleted: {
        if (!app.load_config()) {
            return;
        }

        for (var i = 0; i < app.get_item_count(); i++) {
            iconsModel.append({
                "icon": app.get_item_icon(i),
                "name": app.get_item_name(i)
            });
        }

        if (app.init_gamepad_polling()) {
            gamepadPollTimer.start();
        }
    }

    Image {
        id: bg
        anchors.fill: parent
        fillMode: Image.Stretch
        source: "qrc:/bg.svg"
    }

    ColumnLayout {
        anchors.fill: parent

        Item {
            Layout.fillWidth: true
            Layout.fillHeight: true
        }

        Text {
            id: clock

            Layout.alignment: Qt.AlignHCenter

            Layout.fillHeight: false
            Layout.preferredHeight: window.height * 0.25

            font.pixelSize: height
            font.weight: Font.DemiBold
            font.family: "Ubuntu"
            renderTypeQuality: Text.VeryHighRenderTypeQuality

            color: "#ffffff"
            text: getTime()

            layer.enabled: true
            layer.effect: MultiEffect {
                shadowEnabled: true
                shadowColor: "#80000000"
                shadowVerticalOffset: clock.font.pixelSize * 0.08
                shadowHorizontalOffset: shadowVerticalOffset * 0.5
                shadowBlur: 0.8
            }

            function getTime() {
                var date = new Date;
                var hours = String(date.getHours());
                var minutes = String(date.getMinutes());
                return hours.padStart(2, "0") + ":" + minutes.padStart(2, "0");
            }
        }

        Item {
            Layout.fillWidth: true
            Layout.fillHeight: true
        }

        ListView {
            id: icons

            // Calculate icon size as PowerOfTwo
            property int iconSize: 1 << Math.log2(window.height * 0.15)
            property real labelHeight: iconSize / 4
            property real itemSize: iconSize + labelHeight * 1.2

            Layout.alignment: Qt.AlignHCenter

            Layout.fillHeight: false
            Layout.preferredHeight: itemSize

            orientation: ListView.Horizontal
            Layout.preferredWidth: Math.min(contentWidth, window.width)

            focus: true
            clip: true
            spacing: itemSize * 0.1

            model: ListModel {
                id: iconsModel
            }

            delegate: itemDelegate

            highlight: Rectangle {
                radius: icons.height * 0.05
                color: "#40000000"
            }

            Keys.onReturnPressed: {
                if (app.exec_item(icons.currentIndex)) {
                    icons.interactive = false;
                }
            }

            highlightMoveDuration: 250
            highlightMoveVelocity: -1

            highlightResizeDuration: 250
            highlightResizeVelocity: -1
        }

        Item {
            Layout.fillWidth: true
            Layout.fillHeight: false
            Layout.preferredHeight: window.height * 0.1
        }
    }

    Component {
        id: itemDelegate
        Item {
            id: item

            required property string icon
            required property string name

            width: icons.itemSize
            height: icons.itemSize

            Text {
                id: label

                anchors.horizontalCenter: item.horizontalCenter
                anchors.bottom: item.bottom

                width: icons.itemSize * 0.9
                height: icons.labelHeight

                text: item.name

                font.pixelSize: height * 0.7
                font.weight: Font.DemiBold
                font.family: "Ubuntu"
                color: "#ffffff"

                fontSizeMode: Text.HorizontalFit
                horizontalAlignment: Text.AlignHCenter
            }
            Image {
                id: icon

                anchors.horizontalCenter: item.horizontalCenter
                anchors.bottom: label.top

                width: icons.iconSize
                height: icons.iconSize
                source: "file:" + item.icon
            }
        }
    }

    Timer {
        interval: 250
        running: true
        repeat: true
        onTriggered: {
            clock.text = clock.getTime();
            icons.interactive = !app.has_running_child();
        }
    }

    Timer {
        id: gamepadPollTimer
        interval: 16
        running: false
        repeat: true
        onTriggered: {
            app.poll_gamepad();
        }
    }
}
