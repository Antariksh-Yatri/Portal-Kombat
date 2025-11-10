//
//  wifiautologinApp.swift
//  wifiautologin
//
//  Created by Arjun Manjunath on 06/08/25.
//

import SwiftUI

@main
struct wifiautologinApp: App {
    var body: some Scene {
        MenuBarExtra {
            WiFiCardView()
        } label: {
            Label("Wifi Autologin", systemImage: "wifi.router.fill")
        }.menuBarExtraStyle(.window)
            .windowStyle(.hiddenTitleBar)
    }
}
