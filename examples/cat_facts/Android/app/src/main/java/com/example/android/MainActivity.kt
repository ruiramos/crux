@file:OptIn(ExperimentalUnsignedTypes::class)

package com.example.android

import android.os.Build
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Public
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewmodel.compose.viewModel
import coil.compose.rememberAsyncImagePainter
import com.example.android.ui.theme.AndroidTheme
import com.redbadger.crux_core.shared.*
import com.redbadger.crux_core.shared_types.Msg
import com.redbadger.crux_core.shared_types.PlatformMsg
import com.redbadger.crux_core.shared_types.Request as Req
import com.redbadger.crux_core.shared_types.Requests
import com.redbadger.crux_core.shared_types.RequestBody as ReqBody
import com.redbadger.crux_core.shared_types.Response as Res
import com.redbadger.crux_core.shared_types.ResponseBody as ResBody
import com.redbadger.crux_core.shared_types.ViewModel as MyViewModel
import java.time.ZoneOffset
import java.time.ZonedDateTime
import java.time.format.DateTimeFormatter
import java.util.*
import okhttp3.ResponseBody
import retrofit2.Call
import retrofit2.Callback
import retrofit2.Response
import retrofit2.Retrofit
import retrofit2.http.GET
import retrofit2.http.Url
import kotlin.jvm.optionals.getOrNull

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            AndroidTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) { CatFacts() }
            }
        }
    }
}

fun getPlatform(): String {
    return Build.BRAND + " " + Build.VERSION.RELEASE
}

interface HttpGetService {
    @GET
    fun get(@Url url: String?): Call<ResponseBody?>?

    companion object {
        fun create(): HttpGetService {
            return Retrofit.Builder()
                .baseUrl("http://dummy.com/")
                .build()
                .create(HttpGetService::class.java)
        }
    }
}

sealed class CoreMessage {
    data class Message(val msg: Msg) : CoreMessage()

    data class Response(val res: Res) : CoreMessage()
}

class Model : ViewModel() {
    var view: MyViewModel by mutableStateOf(MyViewModel("", Optional.empty(), ""))
        private set

    init {
        update(CoreMessage.Message(Msg.Get()))
        update(CoreMessage.Message(Msg.Platform(PlatformMsg.Get())))
    }

    private fun httpGet(url: String, uuid: List<Byte>) {
        val call = HttpGetService.create().get(url)
        call?.enqueue(
            object : Callback<ResponseBody?> {
                override fun onResponse(
                    call: Call<ResponseBody?>?,
                    response: Response<ResponseBody?>?
                ) {
                    response?.body()?.bytes()?.toList()?.let { bytes ->
                        update(CoreMessage.Response(Res(uuid, ResBody.Http(bytes))))
                    }
                }

                override fun onFailure(call: Call<ResponseBody?>?, t: Throwable?) {}
            }
        )
    }

    fun update(msg: CoreMessage) {
        val requests: List<Req> =
            when (msg) {
                is CoreMessage.Message -> {
                    Requests.bcsDeserialize(
                        message(msg.msg.bcsSerialize().toUByteArray().toList()).toUByteArray()
                            .toByteArray()
                    )
                }
                is CoreMessage.Response -> {
                    Requests.bcsDeserialize(
                        response(msg.res.bcsSerialize().toUByteArray().toList()).toUByteArray()
                            .toByteArray()
                    )
                }
            }

        for (req in requests) {
            when (val body = req.body) {
                is ReqBody.Render -> {
                    this.view = MyViewModel.bcsDeserialize(view().toUByteArray().toByteArray())
                }
                is ReqBody.Http -> {
                    httpGet(body.value, req.uuid)
                }
                is ReqBody.Time -> {
                    val isoTime =
                        ZonedDateTime.now(ZoneOffset.UTC).format(DateTimeFormatter.ISO_INSTANT)

                    update(CoreMessage.Response(Res(req.uuid, ResBody.Time(isoTime))))
                }
                is ReqBody.Platform -> {
                    val platform = getPlatform()

                    update(CoreMessage.Response(Res(req.uuid, ResBody.Platform(platform))))
                }
                is ReqBody.KVRead -> {
                    update(CoreMessage.Response(Res(req.uuid, ResBody.KVRead(null))))
                }
                is ReqBody.KVWrite -> {
                    update(CoreMessage.Response(Res(req.uuid, ResBody.KVWrite(false))))
                }
            }
        }
    }
}

@OptIn(ExperimentalStdlibApi::class)
@Composable
fun CatFacts(model: Model = viewModel()) {
    Column(
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center,
        modifier = Modifier
            .fillMaxSize()
            .padding(10.dp),
    ) {
        Icon(Icons.Filled.Public, "Platform")
        Text(text = model.view.platform, modifier = Modifier.padding(10.dp))
        Row(
            horizontalArrangement = Arrangement.Center,
            verticalAlignment = Alignment.CenterVertically,
            modifier = Modifier
                .height(250.dp)
                .padding(10.dp)
        ) {
            model.view.image.getOrNull()?.let {
                Image(
                    painter = rememberAsyncImagePainter(it.file),
                    contentDescription = "cat image",
                    modifier = Modifier
                        .height(250.dp)
                        .fillMaxWidth()
                )
            }
        }
        Text(text = model.view.fact, modifier = Modifier.padding(10.dp))
        Row(horizontalArrangement = Arrangement.spacedBy(10.dp)) {
            Button(
                onClick = { model.update(CoreMessage.Message(Msg.Clear())) },
                colors =
                ButtonDefaults.buttonColors(
                    containerColor = MaterialTheme.colorScheme.error
                )
            ) { Text(text = "Clear", color = Color.White) }
            Button(
                onClick = { model.update(CoreMessage.Message(Msg.Get())) },
                colors =
                ButtonDefaults.buttonColors(
                    containerColor = MaterialTheme.colorScheme.primary
                )
            ) { Text(text = "Get", color = Color.White) }
            Button(
                onClick = { model.update(CoreMessage.Message(Msg.Fetch())) },
                colors =
                ButtonDefaults.buttonColors(
                    containerColor = MaterialTheme.colorScheme.secondary
                )
            ) { Text(text = "Fetch", color = Color.White) }
        }
    }
}

@Preview(showBackground = true)
@Composable
fun DefaultPreview() {
    AndroidTheme { CatFacts() }
}
