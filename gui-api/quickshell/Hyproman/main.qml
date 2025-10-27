import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Window 2.15
import QtGraphicalEffects 1.15
import "style" 1.0

ApplicationWindow {
    id: window
    visible: true
    width: 960
    height: 600
    minimumWidth: 720
    minimumHeight: 480
    title: qsTr("HyprDM Display Manager")
    font.family: Style.fontFamily

    property string currentTime: Qt.formatDateTime(new Date(), "HH:mm")
    property string currentDate: Qt.formatDateTime(new Date(), "dddd, d MMMM")
    property string statusMessage: ""
    property color statusColor: "#A0FFCB"
    property bool passwordVisible: false

    function updateDateTime() {
        const now = new Date()
        currentTime = Qt.formatDateTime(now, "HH:mm")
        currentDate = Qt.formatDateTime(now, "dddd, d MMMM")
    }

    function backendAvailable(value) {
        return value !== null && value !== undefined && value !== 0
    }

    function showStatus(message, isError) {
        statusMessage = message
        statusColor = isError ? "#FF8A80" : "#A5D6A7"
        statusResetTimer.restart()
    }

    function attemptLogin() {
        if (!backendAvailable(CompositorBackend) ||
            !backendAvailable(IPCBackend) ||
            !backendAvailable(LayoutManagerBackend) ||
            !backendAvailable(SessionBackend) ||
            !backendAvailable(UserBackend)) {
            showStatus(qsTr("Some services are not ready yet."), true)
            return
        }

        if (backendAvailable(UserBackend) && UserBackend.twofactor_method !== 2) {
            if (!UserBackend.verify_2fa(twoFactorField.text)) {
                showStatus(qsTr("Invalid two-factor code."), true)
                return
            }
        }

        showStatus(qsTr("Starting session…"), false)
        session_start(SessionBackend)
    }

    function restartSession() {
        if (!backendAvailable(SessionBackend)) {
            showStatus(qsTr("Session backend unavailable."), true)
            return
        }
        session_restart(SessionBackend)
        showStatus(qsTr("Session restarted."), false)
    }

    function stopCompositor() {
        if (!backendAvailable(CompositorBackend)) {
            showStatus(qsTr("Compositor backend unavailable."), true)
            return
        }
        compositor_stop(CompositorBackend)
        showStatus(qsTr("Display server stopped."), false)
    }

    Timer {
        interval: 1000
        running: true
        repeat: true
        onTriggered: updateDateTime()
    }

    Timer {
        id: statusResetTimer
        interval: 6000
        repeat: false
        onTriggered: statusMessage = ""
    }

    Image {
        anchors.fill: parent
        source: Style.backgroundImage
        fillMode: Image.PreserveAspectCrop
        smooth: true
        z: -2
    }

    Rectangle {
        anchors.fill: parent
        gradient: Gradient {
            GradientStop { position: 0.0; color: "#BF0F172A" }
            GradientStop { position: 1.0; color: "#C6000B1A" }
        }
        z: -1
    }

    RowLayout {
        anchors.top: parent.top
        anchors.topMargin: 24
        anchors.horizontalCenter: parent.horizontalCenter
        spacing: 24

        Column {
            spacing: 4
            Text {
                text: currentTime
                font.pixelSize: 48
                font.family: Style.fontFamily
                font.bold: true
                color: "#FFFFFF"
            }
            Text {
                text: currentDate
                font.pixelSize: 16
                font.family: Style.fontFamily
                color: "#E0E0E0"
                opacity: 0.85
            }
        }
    }

    Rectangle {
        id: loginCard
        width: Math.min(parent.width * 0.6, 520)
        anchors.verticalCenter: parent.verticalCenter
        anchors.horizontalCenter: parent.horizontalCenter
        radius: Style.borderRadius * 1.5
        color: "#1A0F172A"
        border.color: "#40FFFFFF"
        border.width: 1
        layer.enabled: true
        layer.effect: FastBlur { radius: 24 }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 28
            spacing: 16

            ColumnLayout {
                spacing: 4
                Layout.fillWidth: true

                Text {
                    text: qsTr("Welcome to HyprDM")
                    font.pointSize: 24
                    font.family: Style.fontFamily
                    color: "#FFFFFF"
                }
                Text {
                    text: qsTr("Log into your Wayland session with a refreshed, glassy interface.")
                    font.pointSize: 12
                    font.family: Style.fontFamily
                    color: "#E0E0E0"
                    wrapMode: Text.WordWrap
                    opacity: 0.9
                }
            }

            TextField {
                id: usernameField
                Layout.fillWidth: true
                placeholderText: qsTr("Username")
                font.pointSize: 14
                font.family: Style.fontFamily
                color: Style.textColor
                placeholderTextColor: "#B3FFFFFF"
                leftPadding: 16
                rightPadding: 16
                background: Rectangle {
                    radius: Style.borderRadius
                    color: "#26000000"
                    border.color: control.activeFocus ? Style.primaryColor : "#26FFFFFF"
                    border.width: 1
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                TextField {
                    id: passwordField
                    Layout.fillWidth: true
                    placeholderText: qsTr("Password")
                    echoMode: passwordVisible ? TextInput.Normal : TextInput.Password
                    font.pointSize: 14
                    font.family: Style.fontFamily
                    color: Style.textColor
                    placeholderTextColor: "#B3FFFFFF"
                    leftPadding: 16
                    rightPadding: 16
                    Keys.onReturnPressed: attemptLogin()
                    background: Rectangle {
                        radius: Style.borderRadius
                        color: "#26000000"
                        border.color: control.activeFocus ? Style.primaryColor : "#26FFFFFF"
                        border.width: 1
                    }
                }

                Button {
                    id: togglePassword
                    Layout.preferredWidth: 56
                    text: passwordVisible ? qsTr("Hide") : qsTr("Show")
                    font.family: Style.fontFamily
                    font.pointSize: 12
                    onClicked: passwordVisible = !passwordVisible
                    background: Rectangle {
                        radius: Style.borderRadius
                        color: control.hovered ? Style.hoverColor : Style.primaryColor
                        border.color: "#33FFFFFF"
                        implicitHeight: 40
                        Behavior on color { ColorAnimation { duration: Style.animationDuration } }
                    }
                    contentItem: Text {
                        text: control.text
                        color: "#FFFFFF"
                        font.family: Style.fontFamily
                        font.pointSize: 12
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                }
            }

            TextField {
                id: twoFactorField
                Layout.fillWidth: true
                placeholderText: qsTr("2FA Code")
                visible: backendAvailable(UserBackend) && UserBackend.twofactor_method !== 2
                font.pointSize: 14
                font.family: Style.fontFamily
                color: Style.textColor
                placeholderTextColor: "#B3FFFFFF"
                leftPadding: 16
                rightPadding: 16
                Keys.onReturnPressed: attemptLogin()
                background: Rectangle {
                    radius: Style.borderRadius
                    color: "#26000000"
                    border.color: control.activeFocus ? Style.primaryColor : "#26FFFFFF"
                    border.width: 1
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 12

                ComboBox {
                    id: sessionCombo
                    Layout.fillWidth: true
                    model: [qsTr("Default"), qsTr("Tiling"), qsTr("Floating")]
                    font.family: Style.fontFamily
                    font.pointSize: 14
                    onActivated: showStatus(qsTr("Session set to %1").arg(currentText), false)
                    contentItem: Text {
                        text: control.displayText
                        font.family: Style.fontFamily
                        font.pointSize: 14
                        color: "#FFFFFF"
                        verticalAlignment: Text.AlignVCenter
                        horizontalAlignment: Text.AlignLeft
                        elide: Text.ElideRight
                        leftPadding: 16
                    }
                    background: Rectangle {
                        radius: Style.borderRadius
                        color: "#26000000"
                        border.color: control.activeFocus ? Style.primaryColor : "#26FFFFFF"
                        border.width: 1
                    }
                    delegate: ItemDelegate {
                        width: control.width
                        font.family: Style.fontFamily
                        contentItem: Text {
                            text: modelData
                            font.family: Style.fontFamily
                            font.pointSize: 14
                            color: "#FFFFFF"
                            leftPadding: 16
                            verticalAlignment: Text.AlignVCenter
                        }
                        background: Rectangle {
                            color: control.highlightedIndex === index ? "#3300AAFF" : "#1F000000"
                        }
                    }
                }

                ComboBox {
                    id: themeCombo
                    Layout.fillWidth: true
                    model: ["GTK3", "GTK4", "Qt5", "Qt6", "HyprSensitivity"]
                    font.family: Style.fontFamily
                    font.pointSize: 14
                    onActivated: {
                        if (backendAvailable(ThemeManagerBackend)) {
                            theme_manager_set_theme(ThemeManagerBackend, currentText)
                            theme_manager_load_and_apply(ThemeManagerBackend)
                            showStatus(qsTr("Theme %1 applied.").arg(currentText), false)
                        } else {
                            showStatus(qsTr("Theme manager unavailable."), true)
                        }
                    }
                    contentItem: Text {
                        text: control.displayText
                        font.family: Style.fontFamily
                        font.pointSize: 14
                        color: "#FFFFFF"
                        verticalAlignment: Text.AlignVCenter
                        horizontalAlignment: Text.AlignLeft
                        elide: Text.ElideRight
                        leftPadding: 16
                    }
                    background: Rectangle {
                        radius: Style.borderRadius
                        color: "#26000000"
                        border.color: control.activeFocus ? Style.primaryColor : "#26FFFFFF"
                        border.width: 1
                    }
                    delegate: ItemDelegate {
                        width: control.width
                        font.family: Style.fontFamily
                        contentItem: Text {
                            text: modelData
                            font.family: Style.fontFamily
                            font.pointSize: 14
                            color: "#FFFFFF"
                            leftPadding: 16
                            verticalAlignment: Text.AlignVCenter
                        }
                        background: Rectangle {
                            color: control.highlightedIndex === index ? "#3300AAFF" : "#1F000000"
                        }
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 12

                Button {
                    id: loginButton
                    Layout.fillWidth: true
                    text: qsTr("Login")
                    font.family: Style.fontFamily
                    font.pointSize: 14
                    onClicked: attemptLogin()
                    background: Rectangle {
                        radius: Style.borderRadius
                        color: control.hovered ? Style.hoverColor : Style.primaryColor
                        border.color: "#40FFFFFF"
                        implicitHeight: 44
                        Behavior on color { ColorAnimation { duration: Style.animationDuration } }
                    }
                    contentItem: Text {
                        text: control.text
                        color: "#FFFFFF"
                        font.family: Style.fontFamily
                        font.pointSize: 14
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                }

                Button {
                    id: restartButton
                    Layout.preferredWidth: 120
                    text: qsTr("Restart")
                    font.family: Style.fontFamily
                    font.pointSize: 14
                    onClicked: restartSession()
                    background: Rectangle {
                        radius: Style.borderRadius
                        color: control.hovered ? "#FFB300" : "#FFA000"
                        border.color: "#40FFFFFF"
                        implicitHeight: 44
                        Behavior on color { ColorAnimation { duration: Style.animationDuration } }
                    }
                    contentItem: Text {
                        text: control.text
                        color: "#202020"
                        font.family: Style.fontFamily
                        font.pointSize: 14
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                        font.bold: true
                    }
                }

                Button {
                    id: stopButton
                    Layout.preferredWidth: 120
                    text: qsTr("Stop")
                    font.family: Style.fontFamily
                    font.pointSize: 14
                    onClicked: stopCompositor()
                    background: Rectangle {
                        radius: Style.borderRadius
                        color: control.hovered ? "#EF5350" : "#E53935"
                        border.color: "#40FFFFFF"
                        implicitHeight: 44
                        Behavior on color { ColorAnimation { duration: Style.animationDuration } }
                    }
                    contentItem: Text {
                        text: control.text
                        color: "#FFFFFF"
                        font.family: Style.fontFamily
                        font.pointSize: 14
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                }
            }

            Item {
                Layout.fillWidth: true
                Layout.preferredHeight: statusMessage !== "" ? 24 : 0

                Text {
                    anchors.fill: parent
                    text: statusMessage
                    visible: statusMessage !== ""
                    font.pointSize: 12
                    font.family: Style.fontFamily
                    color: statusColor
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                    wrapMode: Text.WordWrap
                }
            }
        }
    }
}
