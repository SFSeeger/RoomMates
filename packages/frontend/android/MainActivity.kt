package dev.dioxus.main

import androidx.activity.OnBackPressedCallback
import android.content.Intent
import android.net.Uri
import android.os.Bundle
import android.util.Log

// need to re-export buildconfig down from the parent
import io.github.sfseeger.RoomMates.BuildConfig
typealias BuildConfig = BuildConfig

class MainActivity : WryActivity() {
    private external fun nativeHandleGoBack()
    private external fun nativeHandleDeepLink(url: String)

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val callback: OnBackPressedCallback = object : OnBackPressedCallback(true) {
            override fun handleOnBackPressed() {
                nativeHandleGoBack()
            }
        }
        onBackPressedDispatcher.addCallback(this, callback)
        handleIntent(intent)
    }

    override fun onNewIntent(intent: Intent?) {
        Log.d("MainActivity", "Got new intent: $intent")
        super.onNewIntent(intent)
        intent?.let { handleIntent(it) }
    }

    private fun handleIntent(intent: Intent?) {
        val data: Uri? = intent?.data
        Log.d("MainActivity", "Intent data: $data")
        data?.let {
            Log.d("MainActivity", "Calling nativeHandleDeepLink with: ${it.toString()}")
            nativeHandleDeepLink(it.toString())
        }
    }
}
