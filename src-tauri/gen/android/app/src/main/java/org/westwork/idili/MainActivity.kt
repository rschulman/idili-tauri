package org.westwork.idili

import android.content.Context

class MainActivity : TauriActivity() {
    external fun init_android(ctx: Context)
    init {
        init_android(this)
    }
}
