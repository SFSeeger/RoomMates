package dev.dioxus.main
import android.content.Intent
import android.net.Uri
import android.os.Bundle

// need to re-export buildconfig down from the parent
import io.github.sfseeger.RoomMates.BuildConfig
typealias BuildConfig = BuildConfig

class MainActivity : WryActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        handleIntent(intent)
    }

    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        setIntent(intent)
        handleIntent(intent)
    }

    private fun handleIntent(intent: Intent?) {
        val data: Uri? = intent?.data
        data?.let {
            handleDeepLink(it.toString())
        }
    }

    private fun handleDeepLink(url: String) {
        nativeHandleDeepLink(url)
    }

    private external fun nativeHandleDeepLink(url: String)

    companion object {
        init {
            System.loadLibrary("libdioxusmain") // your Rust .so
        }
    }
}
