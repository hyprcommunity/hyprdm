import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Window 2.15

ApplicationWindow {
    visible: true
    width: 800
    height: 600
    title: qsTr("Display Manager")

    // Arka plan
    Rectangle {
        anchors.fill: parent
        color: "#2e3440"
    }

    ColumnLayout {
        anchors.centerIn: parent
        spacing: 20

        // Başlık
        Text {
            text: "Welcome to HyprDM"
            font.pointSize: 24
            color: "#d8dee9"
            horizontalAlignment: Text.AlignHCenter
            Layout.alignment: Qt.AlignHCenter
        }

        // Kullanıcı adı girişi
        TextField {
            id: usernameField
            placeholderText: "Username"
            Layout.preferredWidth: 300
            font.pointSize: 16
        }

        // Şifre girişi
        TextField {
            id: passwordField
            placeholderText: "Password"
            echoMode: TextInput.Password
            Layout.preferredWidth: 300
            font.pointSize: 16
        }

        // Oturum seçimi
        ComboBox {
            id: sessionCombo
            Layout.preferredWidth: 300
            model: ["Default", "Tiling", "Floating"]
        }

        // Butonlar
        RowLayout {
            spacing: 20
            Layout.alignment: Qt.AlignHCenter

            Button {
                text: "Login"
                onClicked: {
                    // Backend fonksiyon çağrısı
                    if (CompositorBackend !== 0 && IPCBackend !== 0) {
                        console.log("Starting session for user:", usernameField.text)
                        session_start(LayoutManagerBackend) // örnek FFI çağrısı
                    }
                }
            }

            Button {
                text: "Restart"
                onClicked: {
                    if (LayoutManagerBackend !== 0) {
                        session_restart(LayoutManagerBackend) // örnek FFI çağrısı
                    }
                }
            }

            Button {
                text: "Stop"
                onClicked: {
                    if (CompositorBackend !== 0) {
                        compositor_stop(CompositorBackend)
                    }
                }
            }
        }

        // Tema değiştirme
        RowLayout {
            spacing: 10
            Layout.alignment: Qt.AlignHCenter

            Button {
                text: "Light Theme"
                onClicked: {
                    if (ThemeManagerBackend !== 0) {
                        theme_manager_set_theme(ThemeManagerBackend, "light")
                        theme_manager_load_and_apply(ThemeManagerBackend)
                    }
                }
            }

            Button {
                text: "Dark Theme"
                onClicked: {
                    if (ThemeManagerBackend !== 0) {
                        theme_manager_set_theme(ThemeManagerBackend, "dark")
                        theme_manager_load_and_apply(ThemeManagerBackend)
                    }
                }
            }
        }
    }
}
