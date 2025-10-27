import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Window 2.15
import QtGraphicalEffects 1.15
import "style" 1.0

ApplicationWindow {
    id: root
    visible: true
    width: 1280
    height: 800
    title: qsTr("HyprDM — Hyproman")
    color: "black"

    property real uiScale: Math.min(width, height) / 900
    property string currentUser: (typeof UserBackend !== "undefined" && UserBackend && UserBackend.username) ? UserBackend.username : ""
    property bool twoFARequired: false
    property string lastError: ""

function updateTwoFA() {
    twoFARequired = false

    if (typeof UserBackend === "undefined" || !UserBackend) {
        console.log("[2FA] UserBackend not available")
        return
    }

    try {
        // 1️⃣ Eğer açıkça 2FA metodu varsa:
        if (UserBackend.hasOwnProperty("twofactor_method")) {
            console.log("[2FA] Detected method =", UserBackend.twofactor_method)
            // method 0 veya 2 -> kapalı
            twoFARequired = (UserBackend.twofactor_method !== 0 && UserBackend.twofactor_method !== 2)
            return
        }

        // 2️⃣ Eğer boolean flag varsa:
        if (UserBackend.hasOwnProperty("twofactor_enabled")) {
            twoFARequired = !!UserBackend.twofactor_enabled
            console.log("[2FA] Enabled flag =", twoFARequired)
            return
        }

        // 3️⃣ Eğer fonksiyon varsa:
        let uname = usernameField.text && usernameField.text.length > 0 ? usernameField.text : currentUser
        if (typeof UserBackend.hasTwoFactor === "function") {
            twoFARequired = !!UserBackend.hasTwoFactor(uname)
            console.log("[2FA] hasTwoFactor() →", twoFARequired)
            return
        }

        if (typeof UserBackend.isTwoFactorEnabled === "function") {
            twoFARequired = !!UserBackend.isTwoFactorEnabled(uname)
            console.log("[2FA] isTwoFactorEnabled() →", twoFARequired)
            return
        }

        // 4️⃣ Hiçbiri yoksa → kapalı
        twoFARequired = false
        console.log("[2FA] No flag found, assuming disabled")

    } catch (e) {
        console.log("[2FA] Exception:", e)
        twoFARequired = false
    }
}

    function verifyPasswordWithBackend(pass) {
        if (!UserBackend) return false
        try {
            if (typeof UserBackend.authenticate === "function")
                return !!UserBackend.authenticate(pass)
        } catch (e) {
            console.log("authenticate() failed:", e)
        }
        return false
    }

    function verifyTwoFAWithBackend(code) {
        if (!twoFARequired) return true
        if (!UserBackend) return false
        try {
            if (typeof UserBackend.verifyTwoFactor === "function")
                return !!UserBackend.verifyTwoFactor(code)
        } catch (e) {
            console.log("verifyTwoFactor() failed:", e)
        }
        return false
    }

    function doLogin() {
        lastError = ""
        var p = passwordField.text
        if (!p || p.length === 0) { lastError = "Password missing"; return }

        var passOk = verifyPasswordWithBackend(p)
        if (!passOk) { lastError = "Invalid password"; return }

        if (twoFARequired) {
            var code = twoFactorField.text
            if (!code || code.length === 0) { lastError = "2FA code required"; return }
            var twoOk = verifyTwoFAWithBackend(code)
            if (!twoOk) { lastError = "Invalid 2FA code"; return }
        }

        console.log("✅ Login success for", currentUser)
    }

    Component.onCompleted: {
        if (currentUser && currentUser.length > 0)
            usernameField.text = currentUser
        updateTwoFA()
    }

    // --- Arka plan ---
    Image {
        id: bg
        anchors.fill: parent
        source: Style.backgroundImage
        fillMode: Image.PreserveAspectCrop
        smooth: true
        z: -2
        SequentialAnimation on scale {
            loops: Animation.Infinite
            NumberAnimation { from: 1.0; to: 1.05; duration: 9000; easing.type: Easing.InOutQuad }
            NumberAnimation { from: 1.05; to: 1.0; duration: 9000; easing.type: Easing.InOutQuad }
        }
        Rectangle {
            anchors.fill: parent
            z: 1
            gradient: Gradient {
                GradientStop { position: 0.0; color: "#66000000" }
                GradientStop { position: 1.0; color: "#AA000000" }
            }
        }
    }

    // --- Restart ikonu ---
    Image {
        id: eyeButton
        anchors.top: parent.top
        anchors.right: parent.right
        anchors.margins: 24 * uiScale
        width: 36 * uiScale
        height: 36 * uiScale
        fillMode: Image.PreserveAspectFit
        opacity: 0.9
        source: "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI2NCIgaGVpZ2h0PSI2NCIgdmlld0JveD0iMCAwIDY0IDY0Ij48cGF0aCBkPSJNNCwzMmM2LTEwLDE4LTE4LDI4LTE4czIyLDgsMjgsMThjLTYsMTAtMTgsMTgtMjgsMThTMTEsNDIsNCwzMnoiIGZpbGw9Im5vbmUiIHN0cm9rZT0id2hpdGUiIHN0cm9rZS13aWR0aD0iMyIvPjxjaXJjbGUgY3g9IjMyIiBjeT0iMzIiIHI9IjgiIGZpbGw9IndoaXRlIi8+PC9zdmc+"
        scale: eyeArea.containsMouse ? 1.15 : 1.0
        Behavior on scale { NumberAnimation { duration: 160; easing.type: Easing.OutQuad } }
        MouseArea {
            id: eyeArea
            anchors.fill: parent
            hoverEnabled: true
            onClicked: restartPopup.open()
        }
        ToolTip.visible: eyeArea.containsMouse
        ToolTip.text: "Restart Session"
    }

    Popup {
        id: restartPopup
        modal: true
        focus: true
        width: 180 * uiScale
        height: 88 * uiScale
        x: eyeButton.x + eyeButton.width - width
        y: eyeButton.y + eyeButton.height + (10 * uiScale)
        opacity: 0.0
        onOpened: NumberAnimation { target: restartPopup; property: "opacity"; from: 0; to: 1; duration: 200 }
        background: Rectangle {
            radius: Style.borderRadius
            color: "#1A000000"
            border.color: Style.borderColor
            border.width: 1.5
        }
        Button {
            anchors.centerIn: parent
            text: "Restart"
            font.pointSize: 14 * uiScale
            background: Rectangle {
                radius: Style.borderRadius
                color: Style.primaryColor
                opacity: hovered ? 1.0 : 0.9
                Behavior on opacity { NumberAnimation { duration: 120 } }
            }
            onClicked: {
                if (typeof SessionBackend !== "undefined" && SessionBackend !== null)
                    session_restart(SessionBackend)
                restartPopup.close()
            }
        }
    }

    // --- Orta panel ---
    Rectangle {
        id: centerPanel
        width: Math.min(560 * uiScale, root.width * 0.5)
        radius: 24 * uiScale
        color: "#1A000000"
        border.color: "#55FFFFFF"
        border.width: 1.2
        anchors.centerIn: parent
        height: formCol.implicitHeight + 56 * uiScale
        layer.enabled: true
        layer.effect: DropShadow {
            horizontalOffset: 0
            verticalOffset: 10
            radius: 25
            samples: 25
            color: "#44000000"
        }

        ColumnLayout {
            id: formCol
            anchors.fill: parent
            anchors.margins: 28 * uiScale
            spacing: 16 * uiScale

            // Başlık
            Rectangle {
                Layout.fillWidth: true
                height: 60 * uiScale
                radius: 18 * uiScale
                color: "#33000000"
                border.color: "#77FFFFFF"
                border.width: 1.0
                Text {
                    anchors.centerIn: parent
                    text: "Welcome to Hyproman"
                    color: "#FFFFFF"
                    font.pixelSize: 22 * uiScale
                    font.bold: true
                    verticalAlignment: Text.AlignVCenter
                }
            }

            // Username
            ColumnLayout {
                Layout.fillWidth: true
                spacing: 6 * uiScale
                Text { text: "Username"; color: "#CCCCCC"; font.pixelSize: 12 * uiScale }
                Rectangle {
                    Layout.fillWidth: true
                    height: 46 * uiScale
                    radius: 12 * uiScale
                    color: "#202020"
                    border.color: "#777"
                    border.width: 1.0
                    TextField {
                        id: usernameField
                        anchors.fill: parent
                        anchors.margins: 12 * uiScale
                        font.pixelSize: 14 * uiScale
                        color: "#FFFFFF"
                        verticalAlignment: Text.AlignVCenter
                        readOnly: currentUser && currentUser.length > 0
                        background: Rectangle { color: "transparent" }
                        onTextChanged: updateTwoFA()
                    }
                }
            }

            // Password
            ColumnLayout {
                Layout.fillWidth: true
                spacing: 6 * uiScale
                Text { text: "Password"; color: "#CCCCCC"; font.pixelSize: 12 * uiScale }
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10 * uiScale
                    Rectangle {
                        Layout.fillWidth: true
                        height: 46 * uiScale
                        radius: 12 * uiScale
                        color: "#202020"
                        border.color: "#777"
                        border.width: 1.0
                        TextField {
                            id: passwordField
                            anchors.fill: parent
                            anchors.margins: 12 * uiScale
                            font.pixelSize: 14 * uiScale
                            color: "#FFFFFF"
                            verticalAlignment: Text.AlignVCenter
                            echoMode: TextInput.Password
                            background: Rectangle { color: "transparent" }
                            Keys.onReturnPressed: loginButton.clicked()
                        }
                    }
                    Button {
                        id: showBtn
                        text: passwordField.echoMode === TextInput.Password ? "Show" : "Hide"
                        width: 72 * uiScale
                        height: 46 * uiScale
                        font.pixelSize: 12 * uiScale
                        background: Rectangle { color: "transparent" }
                        onClicked: passwordField.echoMode =
                            passwordField.echoMode === TextInput.Password ? TextInput.Normal : TextInput.Password
                    }
                }
            }

            // 2FA
            ColumnLayout {
                Layout.fillWidth: true
                spacing: 6 * uiScale
                visible: twoFARequired
                Text { text: "Two-Factor Code"; color: "#CCCCCC"; font.pixelSize: 12 * uiScale }
                Rectangle {
                    Layout.fillWidth: true
                    height: 46 * uiScale
                    radius: 12 * uiScale
                    color: "#202020"
                    border.color: "#777"
                    border.width: 1.0
                    TextField {
                        id: twoFactorField
                        anchors.fill: parent
                        anchors.margins: 12 * uiScale
                        font.pixelSize: 14 * uiScale
                        color: "#FFFFFF"
                        verticalAlignment: Text.AlignVCenter
                        background: Rectangle { color: "transparent" }
                    }
                }
            }

            // Session
            ColumnLayout {
                Layout.fillWidth: true
                spacing: 6 * uiScale
                Text { text: "Session"; color: "#CCCCCC"; font.pixelSize: 12 * uiScale }
                Rectangle {
                    Layout.fillWidth: true
                    height: 46 * uiScale
                    radius: 12 * uiScale
                    color: "#1F00AAFF"
                    border.color: "#66FFFFFF"
                    border.width: 1.0
                    ComboBox {
                        id: sessionCombo
                        anchors.fill: parent
                        anchors.margins: 6 * uiScale
                        model: ["Default", "Tiling", "Floating"]
                        font.pixelSize: 14 * uiScale
                        background: Rectangle { color: "transparent" }
                        contentItem: Text {
                            text: sessionCombo.currentText
                            color: "#FFFFFF"
                            verticalAlignment: Text.AlignVCenter
                            horizontalAlignment: Text.AlignLeft
                            anchors.leftMargin: 10 * uiScale
                        }
                    }
                }
            }

            // Login
            Button {
                id: loginButton
                Layout.fillWidth: true
                height: 50 * uiScale
                text: "Login"
                font.pixelSize: 16 * uiScale
                font.bold: true
                background: Rectangle {
                    radius: 16 * uiScale
                    color: Style.primaryColor
                    opacity: loginButton.hovered ? 1.0 : 0.9
                    Behavior on opacity { NumberAnimation { duration: 140 } }
                }
                onClicked: {
                    doLogin()
                    if (lastError.length > 0)
                        console.warn("Login failed:", lastError)
                }
            }

            // Theme
            ColumnLayout {
                Layout.fillWidth: true
                spacing: 6 * uiScale
                Text { text: "Theme"; color: "#CCCCCC"; font.pixelSize: 12 * uiScale }
                Rectangle {
                    Layout.fillWidth: true
                    height: 46 * uiScale
                    radius: 12 * uiScale
                    color: "#1F00AAFF"
                    border.color: "#66FFFFFF"
                    border.width: 1.0
                    ComboBox {
                        id: themeCombo
                        anchors.fill: parent
                        anchors.margins: 6 * uiScale
                        model: ["GTK3", "GTK4", "Qt5", "Qt6", "HyprSensitivity"]
                        onActivated: {
                            if (typeof Style.applyTheme === "function")
                                Style.applyTheme(themeCombo.currentText)
                        }
                        background: Rectangle { color: "transparent" }
                        contentItem: Text {
                            text: themeCombo.currentText
                            color: "#FFFFFF"
                            verticalAlignment: Text.AlignVCenter
                            horizontalAlignment: Text.AlignLeft
                            anchors.leftMargin: 10 * uiScale
                        }
                    }
                }
            }

            Text {
                text: lastError
                color: lastError.length > 0 ? "#FF6B6B" : "transparent"
                font.pixelSize: 12 * uiScale
                visible: lastError.length > 0
                horizontalAlignment: Text.AlignHCenter
            }
        }
    }
}
