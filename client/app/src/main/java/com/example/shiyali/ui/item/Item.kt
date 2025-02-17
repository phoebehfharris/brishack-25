package com.example.shiyali.ui.item

import androidx.compose.runtime.Composable
import androidx.navigation.NavController

data class Item(
    val id: Int,
    val name: String,
    val description: String,
    val damages: String,
    val imageUrl: String
)