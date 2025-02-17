import android.app.appsearch.SearchResults
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.input.rememberTextFieldState
import androidx.compose.material.MaterialTheme.colors
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.Menu
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material.icons.filled.Search
import androidx.compose.material.icons.filled.ShoppingCart
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.ExpandedFullScreenSearchBar
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.ListItem
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.SearchBar
import androidx.compose.material3.SearchBarDefaults
import androidx.compose.material3.Text
import androidx.compose.material3.rememberSearchBarState
import androidx.compose.material3.SearchBarDefaults.InputField
import androidx.compose.material3.SearchBarValue

import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.painter.Painter
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import coil.compose.AsyncImage
import coil.compose.rememberAsyncImagePainter
import coil.request.ImageRequest
import com.example.shiyali.R
import com.example.shiyali.ui.item.Item
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SearchScreen(navController: NavController) {
    val searchBarState = rememberSearchBarState();
    val textFieldState = rememberTextFieldState();
    val scope = rememberCoroutineScope();

    val results: Array<Item> = arrayOf(
        Item(id=0,name="Grurbler", description = "Cowabunga", damages = "none reported", imageUrl = "https://www.google.com/images/branding/googlelogo/2x/googlelogo_light_color_272x92dp.png")
    );
    val inputField =
        @Composable {
            SearchBarDefaults.InputField(
                modifier = Modifier,
                searchBarState = searchBarState,
                textFieldState = textFieldState,
                onSearch = { scope.launch { searchBarState.animateToCollapsed() } },
                placeholder = { Text("Search...") },
                leadingIcon = {
                    if (searchBarState.currentValue == SearchBarValue.Expanded) {
                        IconButton(
                            onClick = { scope.launch { searchBarState.animateToCollapsed() } }
                        ) {
                            Icon(Icons.AutoMirrored.Default.ArrowBack, contentDescription = "Back")
                        }
                    } else {
                        Icon(Icons.Default.Search, contentDescription = null)
                    }
                },
                trailingIcon = { Icon(Icons.Default.MoreVert, contentDescription = null) },
            )
        }

    Scaffold(
        topBar = {
            SearchBar(
                state = searchBarState,
                inputField = inputField,
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(16.dp)
            )
        }
    ) { padding ->
        ExpandedFullScreenSearchBar(
            state = searchBarState,
            inputField = inputField,
        ) {/*
            LazyColumn {
                results.forEach { item ->
                    item {
                        Card(
                            colors = CardDefaults.cardColors(
                                containerColor = MaterialTheme.colorScheme.onSurfaceVariant
                            ),
                            elevation = CardDefaults.cardElevation(
                                defaultElevation = 6.dp
                            ),
                            shape = RoundedCornerShape(8.dp)
                        ) {
                            Row {
                                AsyncImage(
                                    modifier = Modifier
                                        .size(80.dp, 80.dp)
                                        .clip(RoundedCornerShape(16.dp)),
                                    model = item.imageUrl,
                                    alignment = Alignment.CenterStart,
                                    contentDescription = item.description,
                                )

                                Spacer(modifier = Modifier.width(16.dp))

                                Column(modifier = Modifier.align(Alignment.CenterVertically)) {
                                    Text(item.name)
                                    Spacer(modifier =  Modifier.height(8.dp))
                                    Text(item.description)
                                }
                            }
                        }
                        Spacer(modifier = Modifier)
                    }
                }
            }*/
        }

        LazyColumn(
            verticalArrangement = Arrangement.Center,
            horizontalAlignment = Alignment.CenterHorizontally,
            modifier = Modifier
                .fillMaxWidth()
                .padding(padding),
            contentPadding = PaddingValues(16.dp)
        ) {

            results.forEach { item ->
                item {
                    Card(
                        colors = CardDefaults.cardColors(
                            containerColor = MaterialTheme.colorScheme.surfaceVariant
                        ),
                        elevation = CardDefaults.cardElevation(
                            defaultElevation = 6.dp
                        ),
                        shape = RoundedCornerShape(8.dp),
                        modifier = Modifier.fillMaxWidth()
                            .padding(8.dp)
                            .clip(RoundedCornerShape(16.dp))

                    ) {
                        Row {
                            AsyncImage(
                                modifier = Modifier
                                    .size(80.dp, 80.dp)
                                    .clip(RoundedCornerShape(16.dp)),
                                model = item.imageUrl,
                                alignment = Alignment.CenterStart,
                                contentDescription = item.description,
                                contentScale = ContentScale.Crop,
                                onLoading = {
                                    println("Loading image")
                                },
                                onSuccess = {
                                    println("Success")
                                },
                                onError = { error ->
                                    println(error.result.throwable)
                                }
                            )

                            Spacer(modifier = Modifier.width(16.dp))

                            Column(modifier = Modifier.align(Alignment.CenterVertically)) {
                                Text(item.name)
                                Spacer(modifier =  Modifier.height(8.dp))
                                Text(item.description)
                            }
                        }
                    }
                    Spacer(modifier = Modifier)
                }
            }
        }
    }
}
