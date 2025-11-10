import SwiftUI
import AppKit

struct HoverPointer: ViewModifier {
    func body(content: Content) -> some View {
        content.onHover { hovering in
            if hovering {
                NSCursor.pointingHand.push()
            } else {
                NSCursor.pop()
            }
        }
    }
}

extension View {
    func hoverPointer() -> some View {
        self.modifier(HoverPointer())
    }
}

struct WiFiCardView: View {
    var body: some View {
        VStack(spacing: 18) {
            // Top row: status + settings icons
            HStack {
                HStack(spacing: 10) {
                    Image(systemName: "wifi.slash")
                        .font(.system(size: 18, weight: .semibold))
                        .foregroundColor(.gray.opacity(0.6))
                    VStack(alignment: .leading, spacing: 0) {
                        Text("Disconnected")
                            .font(.system(size: 16, weight: .semibold))
                            .foregroundColor(.gray)
                        Text("Campus WiFi")
                            .font(.system(size: 13))
                            .foregroundColor(.gray.opacity(0.5))
                    }
                }
                Spacer()
                HStack(spacing: 12) {
                    Button { /* left gear */ } label: {
                        Image(systemName: "gearshape.fill")
                            .font(.system(size: 18))
                            .foregroundColor(.gray.opacity(0.5))
                    }
                    .hoverPointer()
                }
            }

            // Action buttons
            VStack(spacing: 12) {
                Button(action: { /* Quick connect action */ }) {
                    HStack {
                        Image(systemName: "wifi")
                            .font(.system(size: 18, weight: .semibold))
                        Text("Quick Connect")
                            .font(.system(size: 18, weight: .semibold))
                    }
                    .frame(maxWidth: .infinity)
                    .padding(.vertical, 14)
                    .foregroundColor(.white)
                }
                .background(Color.blue)
                .cornerRadius(12)
                .hoverPointer()

                Button(action: { /* Manual login action */ }) {
                    HStack {
                        Image(systemName: "person.crop.circle")
                            .font(.system(size: 18, weight: .semibold))
                        Text("Manual Login")
                            .font(.system(size: 16, weight: .semibold))
                    }
                    .frame(maxWidth: .infinity)
                    .padding(.vertical, 12)
                    .foregroundColor(.primary)
                }
                .background(Color.gray.opacity(0.1))
                .cornerRadius(10)
                .hoverPointer()
            }

            Divider()
                .padding(.top, 4)

            // List rows
            VStack(spacing: 18) {
                CardRow(icon: "globe", title: "Network Profiles")
                CardRow(icon: "chart.bar", title: "Usage Statistics")
                CardRow(icon: "clock.arrow.circlepath", title: "Connection History")
            }

        }
        .padding(18)
        .background(
            RoundedRectangle(cornerRadius: 20, style: .continuous)
                .fill(Color(.windowBackgroundColor))
                .shadow(color: .black.opacity(0.06), radius: 20, x: 0, y: 10)
        )
        .overlay(
            RoundedRectangle(cornerRadius: 20)
                .stroke(Color.gray.opacity(0.2), lineWidth: 0.5)
        )
        .frame(maxWidth: 380)
    }
}

private struct CardRow: View {
    let icon: String
    let title: String

    var body: some View {
        HStack(spacing: 14) {
            Image(systemName: icon)
                .font(.system(size: 18))
                .frame(width: 28, height: 28)
                .foregroundColor(.gray)
            Text(title)
                .font(.system(size: 16))
                .foregroundColor(.primary)
            Spacer()
            Image(systemName: "chevron.right")
                .font(.system(size: 16, weight: .semibold))
                .foregroundColor(.gray.opacity(0.4))
        }
    }
}

struct WiFiCardView_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            WiFiCardView()
                .previewLayout(.sizeThatFits)
                .padding()
                .preferredColorScheme(.light)

            WiFiCardView()
                .previewLayout(.sizeThatFits)
                .padding()
                .preferredColorScheme(.dark)
        }
    }
}
