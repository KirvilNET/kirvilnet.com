<script setup lang="ts">
  import { useTemplateRef, onMounted } from 'vue'
  import productCard from '../components/home/product-card.vue'



  const phrases = ['Makers', 'Enterprise', 'Developers', 'Education']

  let currentIndex = 0

  const cyclingText = useTemplateRef('cycling-text')
  //const cursor = useTemplateRef('cursor')

  const typingSpeed = 100 // milliseconds per character
  const deletingSpeed = 100 // milliseconds per character
  const delayBetween = 2000 // milliseconds

  async function typeText(text: any) {
    cyclingText.value!.textContent = ''
    for (let i = 0; i < text.length; i++) {
      cyclingText.value!.textContent += text[i]
      await new Promise((resolve) => setTimeout(resolve, typingSpeed))
    }
  }

  async function deleteText() {
    let text = cyclingText.value!.textContent
    for (let i = text.length; i > 1; i--) {
      cyclingText.value!.textContent = text.substring(0, i - 1)
      await new Promise((resolve) => setTimeout(resolve, deletingSpeed))
    }
  }
  // Main cycling loop
  async function cycle() {
    while (true) {
      await typeText(phrases[currentIndex])
      await new Promise((resolve) => setTimeout(resolve, delayBetween))
      await deleteText()
      currentIndex = (currentIndex + 1) % phrases.length
    }
  }
  onMounted(() => {
    cycle()
  })

</script>

<template>
  <div class="h-full bg-black">
    <!-- Hero -->
    <div class="bg-linear-to-b from-black to-[#8c52ff] via-transparent">
      <div class="max-w-340 mx-auto px-4 sm:px-6 lg:px-8 py-24 space-y-8">
        <!-- Title -->
        <div class="max-w-3xl text-center mx-auto">
          <h1 class="block font-medium text-white text-4xl sm:text-5xl md:text-6xl lg:text-7xl">
            Open-Source Tools for
            <span class="bg-linear-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent inline-block min-h-[1.2em] align-top">
              <span ref="cycling-text">Makers</span>
              <span ref="cursor" class="animate-pulse">|</span>
            </span>
          </h1>
        </div>
        <!-- End Title -->

        <div class="max-w-3xl text-center mx-auto">
          <p class="text-lg text-white/70">Enterprise-grade tools without vendor lock-in.</p>
        </div>

        <!-- Buttons -->
        <div class="text-center">
          <a class="inline-flex justify-center items-center gap-x-3 text-center bg-linear-to-tl from-blue-600 to-violet-600 shadow-lg shadow-transparent hover:shadow-blue-700/50 border border-transparent text-white text-sm font-medium rounded-full focus:outline-hidden focus:shadow-blue-700/50 py-3 px-6" href="https://github.com/KirvilNET/">
            GitHub
            <svg class="shrink-0 size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="m9 18 6-6-6-6" />
            </svg>
          </a>
        </div>
        <!-- End Buttons -->
      </div>
    </div>
    <!-- End Hero -->

    <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-20 border-t border-gray-800">
      <h2 class="text-white text-4xl font-bold mb-12 text-center">Why Choose Kirvilnet</h2>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
        <div class="text-center">
          <div class="text-5xl mb-4">🔓</div>
          <h3 class="text-white text-xl font-bold mb-2">100% Open Source</h3>
          <p class="text-gray-400">MIT licensed. Inspect, modify, and deploy without restrictions.</p>
        </div>
        <div class="text-center">
          <div class="text-5xl mb-4">🚀</div>
          <h3 class="text-white text-xl font-bold mb-2">Production Ready</h3>
          <p class="text-gray-400">Enterprise-grade reliability and performance.</p>
        </div>
        <div class="text-center">
          <div class="text-5xl mb-4">🤝</div>
          <h3 class="text-white text-xl font-bold mb-2">Community Driven</h3>
          <p class="text-gray-400">Built by developers, for developers. Your voice matters in shaping the future of our products.</p>
        </div>
      </div>
    </div>

    <!-- Products Section -->
    <section id="products" class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
      <h2 class="text-white text-4xl font-bold mb-12 text-center">Our Products</h2>

      <div class="grid grid-cols-auto lg:grid-cols-auto gap-8">
        <productCard
          title="QuestSLAM"
          :sub-headers="['Robotics']"
          product-description="Advanced Simultaneous Localization and Mapping system for autonomous robots. QuestSLAM provides robust, real-time spatial awareness for robotic systems."
          :features="['Real-time spatial mapping', 'Highly flexible', 'Blazingly Fast', 'Open source']"
          :tech-stack="['Unity', 'ROS2']"
          product-page-link="/products/questslam"
          image-link="/media/products/QuestSLAM/VR.svg" />
      </div>
    </section>
  </div>
</template>
