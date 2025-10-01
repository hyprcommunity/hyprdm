import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Window 2.15
import QtGraphicalEffects 1.15
import "style" 1.0

ApplicationWindow {
    visible: true
    width: 800
    height: 600
    title: qsTr("Display Manager")
    font.family: "Sans"

    // Arka plan
    Image {
        anchors.fill: parent
        source: Style.backgroundImage
        fillMode: Image.PreserveAspectCrop
        smooth: true
        z: -1
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
            color: "#66000000"
            border.color: Style.borderColor
            radius: Style.borderRadius
            layer.enabled: true
            layer.effect: FastBlur { radius: 16 }
        }

        Button {
            anchors.centerIn: parent
            text: "Restart"
            font.pointSize: 14
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
                if (SessionBackend !== null) session_restart(SessionBackend)
                restartPopup.close()
            }
        }
    }

    ColumnLayout {
        anchors.centerIn: parent
        spacing: 15
        width: 400

        // Başlık
        Rectangle {
            radius: Style.borderRadius
            width: parent.width
            height: 50
            color: "#66000000"
            layer.enabled: true
            layer.effect: FastBlur { radius: 16 }
            border.color: "#FFFFFFFF"
            border.width: 1.8

            Text {
                anchors.centerIn: parent
                text: "Welcome to HyprDM"
                font.pointSize: 20
                font.family: "Sans"
                color: "#FFFFFF"
            }
        }

        // Username
        Rectangle {
            radius: Style.borderRadius
            width: parent.width
            height: 40
            color: "#33000000"
            layer.enabled: true
            layer.effect: FastBlur { radius: 16 }
            border.color: "#FFFFFFFF"
            border.width: 1.5

            TextField {
                id: usernameField
                anchors.fill: parent
                anchors.margins: 5
                placeholderText: "Username"
                font.pointSize: 14
                font.family: "Sans"
                color: "#FFFFFF"
                placeholderTextColor: "#CCCCCC"
                background: Rectangle { color: "transparent" }
            }
        }

        // Password + Show
        RowLayout {
            spacing: 5
            width: parent.width

            Rectangle {
                radius: Style.borderRadius
                width: parent.width - 60
                height: 40
                color: "#33000000"
                layer.enabled: true
                layer.effect: FastBlur { radius: 16 }
                border.color: "#FFFFFFFF"
                border.width: 1.5

                TextField {
                    id: passwordField
                    anchors.fill: parent
                    anchors.margins: 5
                    placeholderText: "Password"
                    echoMode: TextInput.Password
                    font.pointSize: 14
                    font.family: "Sans"
                    color: "#FFFFFF"
                    placeholderTextColor: "#CCCCCC"
                    background: Rectangle { color: "transparent" }
                }
            }

            Button {
                text: "Show"
                font.pointSize: 12
                background: Rectangle { color: "transparent" }
                onClicked: passwordField.echoMode =
                    passwordField.echoMode === TextInput.Password ? TextInput.Normal : TextInput.Password
            }
        }

        // 2FA
        Rectangle {
            radius: Style.borderRadius
            width: parent.width
            height: 40
            color: "#33000000"
            layer.enabled: true
            layer.effect: FastBlur { radius: 16 }
            visible: UserBackend !== null && UserBackend.twofactor_method !== 2
            border.color: "#FFFFFFFF"
            border.width: 1.5

            TextField {
                id: twoFactorField
                anchors.fill: parent
                anchors.margins: 5
                placeholderText: "2FA Code"
                font.pointSize: 14
                font.family: "Sans"
                color: "#FFFFFF"
                placeholderTextColor: "#CCCCCC"
                background: Rectangle { color: "transparent" }
            }
        }

        // Session ComboBox
        Rectangle {
            radius: Style.borderRadius
            width: parent.width
            height: 40
            color: "#3300AAFF"  // Mavi arka plan
            layer.enabled: true
            layer.effect: FastBlur { radius: 16 }
            border.color: "#FFFFFFFF"
            border.width: 1.5

            ComboBox {
                id: sessionCombo
                anchors.fill: parent
                anchors.margins: 5
                model: ["Default", "Tiling", "Floating"]
                font.pointSize: 14
                font.family: "Sans"
                background: Rectangle { color: "transparent" }

                contentItem: Text {
                    text: sessionCombo.currentText
                    font.family: "Sans"
                    font.pointSize: 14
                    color: "#FFFFFF"
                    verticalAlignment: Text.AlignVCenter
                    horizontalAlignment: Text.AlignLeft
                    anchors.left: parent.left
                    anchors.leftMargin: 10
                    elide: Text.ElideRight
                }

                delegate: ItemDelegate {
                    width: sessionCombo.width
                    text: modelData
                    contentItem: Text {
                        text: modelData
                        font.family: "Sans"
                        font.pointSize: 14
                        color: "#FFFFFF"
                        horizontalAlignment: Text.AlignLeft
                        anchors.left: parent.left
                        anchors.leftMargin: 10
                    }
                    background: Rectangle {
                        color: "#3300AAFF"
                        layer.enabled: true
                        layer.effect: FastBlur { radius: 16 }
                    }
                }
            }
        }

        // Login Button
        Rectangle {
            radius: Style.borderRadius
            width: parent.width
            height: 40
            color: "#33000000"
            layer.enabled: true
            layer.effect: FastBlur { radius: 16 }
            border.color: "#FFFFFFFF"
            border.width: 1.5

            Button {
                anchors.fill: parent
                anchors.margins: 5
                text: "Login"
                font.pointSize: 14
                font.family: "Sans"
                background: Rectangle { color: "transparent" }
                contentItem: Text { text: "Login"; color: "#FFFFFF" }
            }
        }

        // Theme ComboBox
        Rectangle {
            radius: Style.borderRadius
            width: parent.width
            height: 40
            color: "#3300AAFF"  // Mavi arka plan
            layer.enabled: true
            layer.effect: FastBlur { radius: 16 }
            border.color: "#FFFFFFFF"
            border.width: 1.5

            ComboBox {
                id: themeCombo
                anchors.fill: parent
                anchors.margins: 5
                model: ["GTK3", "GTK4", "Qt5", "Qt6", "HyprSensitivity"]
                font.pointSize: 14
                font.family: "Sans"
                background: Rectangle { color: "transparent" }

                contentItem: Text {
                    text: themeCombo.currentText
                    font.family: "Sans"
                    font.pointSize: 14
                    color: "#FFFFFF"
                    verticalAlignment: Text.AlignVCenter
                    horizontalAlignment: Text.AlignLeft
                    anchors.left: parent.left
                    anchors.leftMargin: 10
                    elide: Text.ElideRight
                }

                delegate: ItemDelegate {
                    width: themeCombo.width
                    text: modelData
                    contentItem: Text {
                        text: modelData
                        font.family: "Sans"
                        font.pointSize: 14
                        color: "#FFFFFF"
                        horizontalAlignment: Text.AlignLeft
                        anchors.left: parent.left
                        anchors.leftMargin: 10
                    }
                    background: Rectangle {
                        color: "#3300AAFF"
                        layer.enabled: true
                        layer.effect: FastBlur { radius: 16 }
                    }
                }
            }
        }

    } // ColumnLayout
} // ApplicationWindow
