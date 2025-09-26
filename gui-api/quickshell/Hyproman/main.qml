import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Window 2.15

ApplicationWindow {
    visible: true
    width: 800
    height: 600
    title: qsTr("Display Manager")

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

        // İki faktör kodu (sadece aktifse göster)
        TextField {
            id: twoFactorField
            placeholderText: "2FA Code"
            Layout.preferredWidth: 300
            font.pointSize: 16
            visible: UserBackend !== 0 && UserBackend.twofactor_method !== 2 // 2 = None
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
                    if (CompositorBackend !== 0 && IPCBackend !== 0 && LayoutManagerBackend !== 0 && SessionBackend !== 0 && UserBackend !== 0) {
                        // 2FA aktifse kontrol et
                        if (UserBackend.twofactor_method !== 2) {
                            if (UserBackend.verify_2fa(twoFactorField.text)) {
                                console.log("2FA verified")
                                session_start(SessionBackend)
                            } else {
                                console.log("Invalid 2FA code")
                            }
                        } else {
                            session_start(SessionBackend)
                        }
                    }
                }
            }

            Button {
                text: "Restart"
                onClicked: {
                    if (SessionBackend !== 0) {
                        session_restart(SessionBackend)
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

        // Tema değiştirme (GTK, Qt, HyprSensitivity)
        ComboBox {
            id: themeCombo
            Layout.preferredWidth: 300
            model: ["GTK3", "GTK4", "Qt5", "Qt6", "HyprSensitivity"]

            onCurrentTextChanged: {
                if (ThemeManagerBackend !== 0) {
                    theme_manager_set_theme(ThemeManagerBackend, currentText)
                    theme_manager_load_and_apply(ThemeManagerBackend)
                }
            }
        }
    }
}
