import QtQuick 2.15
import QtQuick.Controls 2.15

ApplicationWindow {
    visible: true
    width: 800
    height: 600
    title: "Hyproman"

    Column {
        anchors.centerIn: parent
        spacing: 10

        ComboBox {
            id: sessionBox
            width: 200
            // Rust backend'den alınan session listesi
            model: BackendAPI.availableSessions()
        }

        Button {
            text: "Switch Session"
            onClicked: {
                var success = BackendAPI.switchSession(sessionBox.currentText)
                console.log("Switch session:", success)
            }
        }

        ComboBox {
            id: themeBox
            width: 200
            // Rust backend'den alınacak tema listesi, şimdilik placeholder
            model: ["Light", "Dark", "Custom"]
        }

        Button {
            text: "Apply Theme"
            onClicked: {
                var success = BackendAPI.setTheme(themeBox.currentText)
                console.log("Apply theme:", success)
            }
        }

        Button {
            text: "Apply Layout"
            onClicked: BackendAPI.applyLayout()
        }

        Text {
            id: activeWin
            text: "Active window: " + BackendAPI.activeWindow()
        }

        Timer {
            interval: 1000
            running: true
            repeat: true
            onTriggered: activeWin.text = "Active window: " + BackendAPI.activeWindow()
        }
    }
}
