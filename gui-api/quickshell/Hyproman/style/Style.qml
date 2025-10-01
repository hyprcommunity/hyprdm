pragma Singleton
import QtQuick 2.15
import QtGraphicalEffects 1.15

QtObject {
    property color baseColor: "#5C6BC0"
    property color primaryColor: baseColor
    property color hoverColor: Qt.lighter(baseColor, 1.3)
    property color secondaryColor: Qt.darker(baseColor, 1.5)
    property color textColor: "white"
    property int borderRadius: 12
    property int animationDuration: 250
    property string fontFamily: "Sans"
    property url backgroundImage: "../resource/Wallpapers/background.jpg"

    // Blur panel component
    property Component blurPanel: Rectangle {
        id: panelRect
        radius: Style.borderRadius
        color: "#ffffff88"   // Daha kontrastlı ve okunabilir
        layer.enabled: true
        layer.smooth: true

        FastBlur {
            anchors.fill: parent
            radius: 16
            source: panelRect
        }

        DropShadow {
            anchors.fill: parent
            horizontalOffset: 0
            verticalOffset: 2
            radius: 8
            samples: 16
            color: "#00000050"
        }
    }

    // Hover button component
    property Component hoverButton: Rectangle {
        id: btnRect
        radius: Style.borderRadius
        color: Style.primaryColor

        MouseArea {
            anchors.fill: parent
            hoverEnabled: true
            onEntered: btnRect.color = Style.hoverColor
            onExited: btnRect.color = Style.primaryColor
        }

        Behavior on color {
            ColorAnimation { duration: Style.animationDuration }
        }
    }
}
