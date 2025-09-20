//
//  ContentView.swift
//  wifiautologin
//
//  Created by Arjun Manjunath on 06/08/25.
//

import SwiftUI


struct ConnectionStatus: View {
    var body: some View {
        HStack {
            Image(systemName: "wifi.slash").foregroundColor(.gray)
            VStack{
                Text("Disconnected").foregroundColor(.black)
                Text("Campus Wifi").foregroundColor(.black)
            }
        }
    }
}

struct PopupView: View {
    @State private var isHovered = false

    var body: some View {
        VStack(spacing: 5) {
            HStack {
                Image(systemName: "wifi.slash").foregroundColor(.gray)
                VStack{
                    Text("Disconnected")
                        .font(.headline)
                        .foregroundColor(.gray)
                    Text("Campus WiFi")
                        .font(.headline)
                        .foregroundColor(.gray)
                }.padding()
                Spacer()
                Image(systemName: "gearshape.fill").foregroundColor(.gray)
            }
    
            
            
            // Quick Connect Button
            Button(action: {
                // Handle Quick Connect action
            }) {
                HStack {
                    Image(systemName: "wifi").foregroundColor(.white)
                    Text("Quick Connect")
                        .fontWeight(.bold)
                        .foregroundColor(.white)
                    
                }
                .padding()
                .background(.clear)
                
            }
            .frame(maxWidth: .infinity)
            .background(Color.blue)
            .cornerRadius(10)
            .buttonStyle(PlainButtonStyle())
//            .shadow(color: .gray, radius: 5, x: 0, y: 4)

            
            
            // Manual Login Button
            Button(action: {
                // Handle Manual Login action
            }) {
                HStack {
                    Image(systemName: "person.fill").foregroundColor(.black)
                    Text("Manual Login")
                        .fontWeight(.semibold)
                        .foregroundColor(.black)

                }
                .padding()
                .cornerRadius(10)
                
            }
            .frame(maxWidth: .infinity)
            
            .buttonStyle(PlainButtonStyle())
            .background(isHovered ? Color.blue : Color(cgColor: CGColor(red: 243.0, green: 244.0, blue: 246.0, alpha: 255)))
            .shadow(color: .black.opacity(0.3), radius: 8, x: 0, y: 4)
            .cornerRadius(10)
            .transition(.opacity) // Add transition
                           .animation(.easeInOut(duration: 0.3), value: isHovered) // Animation for smooth transition
                           .onHover { hovering in
                               withAnimation {
                                   isHovered = hovering
                               }
                           }
            Divider()
                .frame(height: 1)
                .background(Color.gray)
                .opacity(0.2)
            // Other Options
            VStack(spacing: 15) {
                Button(action: {
                    // Handle Network Profiles action
                }) {
                    HStack {
                        Image(systemName: "network")
                        Text("Network Profiles")
                        Spacer()
                        Image(systemName: "chevron.right")
                    }
                    .padding()
                    .foregroundColor(.gray)
                }
                .buttonStyle(PlainButtonStyle())

                
                Button(action: {
                    // Handle Usage Statistics action
                }) {
                    HStack {
                        Image(systemName: "chart.bar.xaxis")
                        Text("Usage Statistics")
                        Spacer()
                        Image(systemName: "chevron.right")
                    }
                    .padding()
                    .foregroundColor(.gray)
                }
                .buttonStyle(PlainButtonStyle())

                
                Button(action: {
                    // Handle Connection History action
                }) {
                    HStack {
                        Image(systemName: "clock.arrow.circlepath")
                        Text("Connection History")
                        Spacer()
                        Image(systemName: "chevron.right")
                    }
                    .padding()
                    .foregroundColor(.gray)
                    .background(.white)
                }
                .buttonStyle(PlainButtonStyle())

            }
        }
        .padding()
        .edgesIgnoringSafeArea(.all)
        .frame(width: 300)
        .background(.white)
    }
}

#Preview {
    PopupView()
}
