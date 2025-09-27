import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Window 2.15
import "style" 1.0 // Style.qml singleton import

ApplicationWindow {
    visible: true
    width: 800
    height: 600
    title: qsTr("Display Manager")

    Rectangle {
        anchors.fill: parent
        color: Style.gradientEnd
    }

    // Sağ üst göz simgesi
    Button {
        id: eyeButton
        text: "\u{1F441}"
        font.pixelSize: 24
        anchors.top: parent.top
        anchors.right: parent.right
        anchors.margins: 20
        background: Rectangle { color: "transparent" }

        onClicked: restartPopup.open()
    }

    // Restart popup
    Popup {
        id: restartPopup
        modal: true
        focus: true
        x: eyeButton.x + eyeButton.width - width
        y: eyeButton.y + eyeButton.height + 5
        width: 120
        height: 50
        background: Rectangle {
            color: Style.secondaryColor
            border.color: Style.borderColor
            radius: Style.borderRadius
        }

        Button {
            anchors.centerIn: parent
            text: "Restart"
            font.pointSize: Style.fontSizeMedium
            background: Rectangle {
                radius: Style.borderRadius
                color: Style.primaryColor
            }
            MouseArea {
                anchors.fill: parent
                hoverEnabled: true
                onEntered: parent.color = Style.hoverColor
                onExited: parent.color = Style.primaryColor
            }
            onClicked: {
                if (SessionBackend !== 0) session_restart(SessionBackend)
                restartPopup.close()
            }
        }
    }

    ColumnLayout {
        anchors.centerIn: parent
        spacing: 20

        // Başlık
        Text {
            text: "Welcome to HyprDM"
            font.pointSize: Style.fontSizeLarge
            color: Style.textColor
            horizontalAlignment: Text.AlignHCenter
            Layout.alignment: Qt.AlignHCenter
        }

        // Kullanıcı adı girişi
        TextField {
            id: usernameField
            placeholderText: "Username"
            Layout.preferredWidth: 300
            font.pointSize: Style.fontSizeMedium
            color: Style.textColor
            background: Rectangle {
                radius: Style.borderRadius
                color: Style.secondaryColor
                border.color: Style.borderColor
                border.width: 1
            }
        }

        // Şifre girişi + göster/gizle butonu
        RowLayout {
            Layout.preferredWidth: 300
            spacing: 5

            TextField {
                id: passwordField
                placeholderText: "Password"
                echoMode: TextInput.Password
                Layout.fillWidth: true
                font.pointSize: Style.fontSizeMedium
                color: Style.textColor
                background: Rectangle {
                    radius: Style.borderRadius
                    color: Style.secondaryColor
                    border.color: Style.borderColor
                    border.width: 1
                }
            }

            Button {
                text: "Show"
                font.pointSize: 12
                onClicked: passwordField.echoMode = passwordField.echoMode === TextInput.Password ? TextInput.Normal : TextInput.Password
            }
        }

        // İki faktör kodu (sadece aktifse göster)
        TextField {
            id: twoFactorField
            placeholderText: "2FA Code"
            Layout.preferredWidth: 300
            font.pointSize: Style.fontSizeMedium
            color: Style.textColor
            background: Rectangle {
                radius: Style.borderRadius
                color: Style.secondaryColor
                border.color: Style.borderColor
                border.width: 1
            }
            visible: UserBackend !== 0 && UserBackend.twofactor_method !== 2
        }

        // Oturum seçimi
        ComboBox {
            id: sessionCombo
            Layout.preferredWidth: 300
            model: ["Default", "Tiling", "Floating"]
            background: Rectangle {
                radius: Style.borderRadius
                color: Style.secondaryColor
                border.color: Style.borderColor
                border.width: 1
            }
        }

        // Login butonu
        Button {
            text: "Login"
            font.pointSize: Style.fontSizeMedium
            background: Rectangle {
                radius: Style.borderRadius
                color: Style.primaryColor
            }
            onClicked: {
                if (CompositorBackend !== 0 && IPCBackend !== 0 && LayoutManagerBackend !== 0 && SessionBackend !== 0 && UserBackend !== 0) {
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

        // Tema değiştirme (GTK, Qt, HyprSensitivity)
        ComboBox {
            id: themeCombo
            Layout.preferredWidth: 300
            model: ["GTK3", "GTK4", "Qt5", "Qt6", "HyprSensitivity"]
            background: Rectangle {
                radius: Style.borderRadius
                color: Style.secondaryColor
                border.color: Style.borderColor
                border.width: 1
            }
            onCurrentTextChanged: {
                if (ThemeManagerBackend !== 0) {
                    theme_manager_set_theme(ThemeManagerBackend, currentText)
                    theme_manager_load_and_apply(ThemeManagerBackend)
                }
            }
        }
    }
}
