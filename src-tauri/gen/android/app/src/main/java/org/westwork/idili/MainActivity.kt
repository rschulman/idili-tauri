package org.westwork.idili

import android.content.Context
import android.os.Bundle

class MainActivity : TauriActivity() {
    external fun init_android(ctx: Context)
    external fun filelocation(path: String)
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        init_android(this)
        filelocation(this.filesDir.absolutePath)
    }
}
