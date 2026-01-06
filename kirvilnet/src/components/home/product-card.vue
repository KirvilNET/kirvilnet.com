<script setup lang="ts">
  //import { ref } from 'vue'

  const props = defineProps<{
    title: string
    subHeaders: string[]
    productDescription: string
    features: string[]
    techStack: string[]
    productPageLink: string
    imageLink: string
  }>()
</script>

<template>
  <div class="group relative">
    <div
      class="absolute inset-0 bg-linear-to-r from-blue-500/20 to-purple-500/20 rounded-xl blur-xl group-hover:blur-2xl transition-all opacity-0 group-hover:opacity-100">
    </div>
    <div
      class="relative bg-gray-900/50 border border-gray-800 group-hover:border-blue-500/50 rounded-xl p-8 transition-all">
      <!-- Header -->
      <div class="flex items-start justify-between mb-4">
        <div>
          <h3 class="text-white text-2xl font-bold mb-2">{{title}}</h3>
          <div v-for="header in subHeaders">
            <span class="inline-block px-3 py-1 bg-blue-500/20 border border-blue-500/50 rounded-full text-sm font-medium text-blue-300">
              {{header}}
            </span>
          </div>
        </div>
        <div class="text-4xl"><img class="w-19 h-19" :src="imageLink"></img></div>
      </div>
      <!-- Description -->
      <p class="text-gray-400 mb-6">
        {{ productDescription }}
      </p>
      <!-- Features -->
      <div class="space-y-2 mb-6">
        <div v-for="feature in features">
          <div class="flex items-center gap-2 text-sm text-gray-300">
            <span class="text-blue-400">✓</span> {{ feature }}
          </div>
        </div>
      </div>
      <!-- Tech Stack -->
      <div class="flex flex-wrap gap-2 mb-6">
        <div v-for="item in techStack">
          <span class="px-2 py-1 bg-gray-800 rounded text-xs text-gray-300">{{item}}</span>
        </div>
      </div>
      <!-- CTA -->
      <router-link :to="productPageLink" class="inline-flex items-center gap-2 px-4 py-2 border border-blue-500/50 hover:border-blue-400 hover:bg-blue-500/10 rounded-lg font-medium transition-colors text-blue-300">
        Learn More <span>→</span>
      </router-link>
    </div>
  </div>
</template>
