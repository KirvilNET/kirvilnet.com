<script setup lang="ts">
  import subNav from '../../components/nav/subNavBar.vue'
  import type { PageSections } from '../../components/Objects.ts'

  const QuestSLAMPageSections: PageSections[] = [
    {pageName: "overview", buttonName: "Overview"},
    {pageName: "features", buttonName: "Features"},
    {pageName: "upcoming-features", buttonName: "Upcoming Features"},
    {pageName: "faq", buttonName: "FAQ"},
  ]

  function scrollToSection(section: string) {
    document.getElementById(section)?.scrollIntoView({ behavior: 'smooth' });
  }

</script>

<template>
  <div class="bg-black">
    <subNav
      -product-title="QuestSLAM"
      -product-catagory="Robotics"
      :page-sections="QuestSLAMPageSections"
    />
		<!-- Hero div -->
    <div id="hero" class="relative overflow-hidden">
        <div class="absolute inset-0 -z-10">
            <div class="absolute top-0 left-1/4 w-96 h-96 bg-blue-500/10 rounded-full blur-3xl animate-pulse"></div>
            <div class="absolute bottom-0 right-1/4 w-96 h-96 bg-purple-500/10 rounded-full blur-3xl animate-pulse" style="animation-delay: 1s;"></div>
        </div>
        <div class=" max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
            <div class="text-center mb-16">
                <h1 class="text-white text-5xl sm:text-6xl font-bold mb-6">
                    A New Take on
                    <span class="bg-linear-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
                        SLAM
                    </span>
                </h1>
                <p class="text-xl text-gray-400 max-w-2xl mx-auto">
                    Open-source SLAM and spatial mapping for robotics, powered by Meta Quest 3 and ROS 2.
                </p>

                <div class="mt-6 flex flex-col sm:flex-row gap-4 justify-center">
                    <a href="https://github.com/KirvilNET/QuestSLAM" class="text-white px-8 py-3 bg-linear-to-r from-blue-500 to-purple-500 hover:from-blue-600 hover:to-purple-600 rounded-lg font-semibold transition-all duration-300 hover:shadow-lg hover:shadow-blue-500/50">
                        Get Started
                    </a>
                </div>
            </div>
        </div>
    </div>
    <div id="overview" class="relative isolate overflow-hidden px-6 py-24 sm:py-32 lg:overflow-visible lg:px-0">
      <div class="absolute inset-0 -z-10 overflow-hidden">
        <svg aria-hidden="true" class="absolute top-0 left-[max(50%,25rem)] h-256 w-512 -translate-x-1/2 mask-[radial-gradient(64rem_64rem_at_top,white,transparent)] stroke-gray-800">
          <defs>
            <pattern id="e813992c-7d03-4cc4-a2bd-151760b470a0" width="200" height="200" x="50%" y="-1" patternUnits="userSpaceOnUse">
              <path d="M100 200V.5M.5 .5H200" fill="none" />
            </pattern>
          </defs>
          <svg x="50%" y="-1" class="overflow-visible fill-gray-800/50">
            <path d="M-100.5 0h201v201h-201Z M699.5 0h201v201h-201Z M499.5 400h201v201h-201Z M-300.5 600h201v201h-201Z" stroke-width="0" />
          </svg>
          <rect width="100%" height="100%" fill="url(#e813992c-7d03-4cc4-a2bd-151760b470a0)" stroke-width="0" />
        </svg>
      </div>
      <div class="mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 lg:mx-0 lg:max-w-none lg:grid-cols-2 lg:items-start lg:gap-y-10">
        <div class="lg:col-span-2 lg:col-start-1 lg:row-start-1 lg:mx-auto lg:grid lg:w-full lg:max-w-7xl lg:grid-cols-2 lg:gap-x-8 lg:px-8">
          <div class="lg:pr-4">
            <div class="lg:max-w-lg">
              <p class="text-base/7 font-semibold text-[#8c52ff]">Technical</p>
              <h1 class="mt-2 text-4xl font-semibold tracking-tight text-pretty text-white sm:text-5xl">How does it work?</h1>
              <p class="mt-6 text-xl/8 text-gray-300">QuestSLAM works by leveraging Unity's rich intergration with the Meta SDK to obtain tracking data. Our software handles the conversion from Unity coordinates to ROS2 coordinates, Image Streaming, AprilTag tracking, and other features.</p>
            </div>
          </div>
        </div>
        <div class="-mt-12 -ml-12 p-12 lg:col-start-2 lg:row-span-2 lg:row-start-1 lg:overflow-hidden">
          <img src="/media/products/QuestSLAM/Headset Tracking Data.svg" alt="" class="w-3xl max-w-none rounded-xl bg-gray-800 shadow-xl ring-1 ring-white/20 sm:w-228" />
        </div>
      </div>
    </div>
    <!-- Features div -->
    <div id="features" class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-20 border-t border-gray-800">
        <h2 class="text-white text-4xl font-bold mb-12 text-center">Key Features</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8 mb-16">
            <!-- Feature Card 1 -->
            <div class="group relative">
                <div class="absolute inset-0 bg-linear-to-r from-blue-500/20 to-purple-500/20 rounded-xl blur-xl group-hover:blur-2xl transition-all opacity-0 group-hover:opacity-100"></div>
                <div class="relative bg-grey-900/50 bg-[#8c52ff] border border-gray-800 group-hover:border-blue-500/50 rounded-xl p-8 transition-all">
                    <div class="text-3xl mb-4">📍</div>
                    <h3 class="text-white text-xl font-bold mb-2">Impressive Tracking</h3>
                    <p class="text-white">120Hz tracking rate with ±1cm accuracy using VIO technology. Real-time spatial awareness for precise robot navigation.</p>
                </div>
            </div>
            <!-- Feature Card 2 -->
            <div class="group relative">
                <div class="absolute inset-0 bg-linear-to-r from-blue-500/20 to-purple-500/20 rounded-xl blur-xl group-hover:blur-2xl transition-all opacity-0 group-hover:opacity-100"></div>
                <div class="relative bg-grey-900/50 bg-[#8c52ff] border border-gray-800 group-hover:border-blue-500/50 rounded-xl p-8 transition-all">
                    <div class="text-3xl mb-4">💰</div>
                    <h3 class="text-white text-xl font-bold mb-2">Cost Effective</h3>
                    <p class="text-white">Uses commercially available hardware with optional 3D-printed components. Accessible to hobbyists and enterprises alike.</p>
                </div>
            </div>
            <!-- Feature Card 3 -->
            <div class="group relative">
                <div class="absolute inset-0 bg-linear-to-r from-blue-500/20 to-purple-500/20 rounded-xl blur-xl group-hover:blur-2xl transition-all opacity-0 group-hover:opacity-100"></div>
                <div class="relative bg-grey-900/50 bg-[#8c52ff] border border-gray-800 group-hover:border-blue-500/50 rounded-xl p-8 transition-all">
                    <div class="text-3xl mb-4">🔌</div>
                    <h3 class="text-white text-xl font-bold mb-2">Easy Integration</h3>
                    <p class="text-white">Designed for ROS 2. Simple setup, straightforward operation, and seamless integration with existing systems.</p>
                </div>
            </div>
            <!-- Feature Card 4 -->
            <div class="group relative">
                <div class="absolute inset-0 bg-linear-to-r from-blue-500/20 to-purple-500/20 rounded-xl blur-xl group-hover:blur-2xl transition-all opacity-0 group-hover:opacity-100"></div>
                <div class="relative bg-grey-900/50 bg-[#8c52ff] border border-gray-800 group-hover:border-blue-500/50 rounded-xl p-8 transition-all">
                    <div class="text-3xl mb-4">🎁</div>
                    <h3 class="text-white text-xl font-bold mb-2">Self-Contained</h3>
                    <p class="text-white">Quest 3 provides everything: IMU, color cameras, rechargeable battery, and powerful computing in one device.</p>
                </div>
            </div>
            <!-- Feature Card 5 -->
            <div class="group relative">
                <div class="absolute inset-0 bg-linear-to-r from-blue-500/20 to-purple-500/20 rounded-xl blur-xl group-hover:blur-2xl transition-all opacity-0 group-hover:opacity-100"></div>
                <div class="relative bg-grey-900/50 bg-[#8c52ff] border border-gray-800 group-hover:border-blue-500/50 rounded-xl p-8 transition-all">
                    <div class="text-3xl mb-4">🔓</div>
                    <h3 class="text-white text-xl font-bold mb-2">Open Source</h3>
                    <p class="text-white">MIT licensed. Always free and open-source. Customize and adapt to your specific needs.</p>
                </div>
            </div>
            <!-- Feature Card 6 -->
            <div class="group relative">
                <div class="absolute inset-0 bg-linear-to-r from-blue-500/20 to-purple-500/20 rounded-xl blur-xl group-hover:blur-2xl transition-all opacity-0 group-hover:opacity-100"></div>
                <div class="relative bg-grey-900/50 bg-[#8c52ff] border border-gray-800 group-hover:border-blue-500/50 rounded-xl p-8 transition-all">
                    <div class="text-3xl mb-4">⚙️</div>
                    <h3 class="text-white text-xl font-bold mb-2">Unity Framework</h3>
                    <p class="text-white">Built on Unity for powerful rendering, physics simulation, and a flexible application framework.</p>
                </div>
            </div>
        </div>
    </div>
    <!-- Upcoming Features -->
    <div id="upcoming-features" class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-20 border-t border-gray-800">
        <h2 class="text-white text-4xl font-bold mb-12 text-center">Upcoming Features</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
            <div class="bg-grey-900/50 bg-[#8c52ff] border border-gray-800 rounded-xl p-6">
                <h3 class="text-white text-lg font-bold mb-2">AprilTag Tracking</h3>
                <p class="text-white">Track AprilTags and display them in RViz. Use markers as local positioning anchors for precise localization.</p>
            </div>
            <div class="bg-grey-900/50 bg-[#8c52ff] border border-gray-800 rounded-xl p-6">
                <h3 class="text-white text-lg font-bold mb-2">Video Streaming</h3>
                <p class="text-white">Stream Quest 3 camera feeds over your network connection for remote monitoring and debugging.</p>
            </div>
            <div class="bg-grey-900/50 bg-[#8c52ff] border border-gray-800 rounded-xl p-6">
                <h3 class="text-white text-lg font-bold mb-2">Web Dashboard</h3>
                <p class="text-white">Centralized configuration, plugin management, camera feeds, and system statistics in one place.</p>
            </div>
            <div class="bg-grey-900/50 bg-[#8c52ff] border border-gray-800 rounded-xl p-6">
                <h3 class="text-white text-lg font-bold mb-2">Plugin System</h3>
                <p class="text-white">Extend QuestSLAM with custom plugins for new functionality and data transport methods.</p>
            </div>
        </div>
    </div>
    <!-- FAQ div -->
    <div id="faq" class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-20 border-t border-gray-800">
        <h2 class="text-white text-4xl font-bold mb-12 text-center">Frequently Asked Questions</h2>
        <div class="space-y-6">
            <details class="bg-grey-900/50 bg-[#8c52ff] border border-gray-800 rounded-xl p-6 group cursor-pointer">
                <summary class="flex items-center justify-between font-bold text-white text-lg hover:text-purple-400 transition">
                    How do I integrate QuestSLAM into my project?
                    <span class="color-white transition group-open:rotate-180">▼</span>
                </summary>
                <p class="mt-4 text-white">Integration is straightforward using our ROS 2 package. Follow the documentation to get up and running in minutes. We provide detailed setup instructions and example configurations.</p>
            </details>
            <details class="bg-grey-900/50 bg-[#8c52ff] border border-gray-800 rounded-xl p-6 group cursor-pointer">
                <summary class="flex items-center justify-between font-bold text-white text-lg hover:text-purple-400 transition">
                    Can I use QuestSLAM without ROS 2?
                    <span class="color-white transition group-open:rotate-180">▼</span>
                </summary>
                <p class="mt-4 text-white">Currently, ROS 2 integration is required. However, we're actively working on a standalone version that will be available soon, allowing usage without ROS 2 dependencies.</p>
            </details>
            <details class="bg-grey-900/50 bg-[#8c52ff] border border-gray-800 rounded-xl p-6 group cursor-pointer">
                <summary class="flex items-center justify-between font-bold text-white text-lg hover:text-purple-400 transition">
                    What hardware do I need?
                    <span class="color-white transition group-open:rotate-180">▼</span>
                </summary>
                <p class="mt-4 text-white">You need a Meta Quest 3 headset and a computer running ROS 2 on Linux (Ubuntu 22.04 or 24.04 recommended). The Quest and computer communicate over your network via ROS Bridge.</p>
            </details>
            <details class="bg-grey-900/50 bg-[#8c52ff] border border-gray-800 rounded-xl p-6 group cursor-pointer">
                <summary class="flex items-center justify-between font-bold text-white text-lg hover:text-purple-400 transition">
                    What's the accuracy and tracking rate?
                    <span class="color-white transition group-open:rotate-180">▼</span>
                </summary>
                <p class="mt-4 text-white">QuestSLAM achieves ±1cm positional accuracy with a 120Hz tracking rate using Visual Inertial Odometry (VIO) from the Quest 3's built-in sensors.</p>
            </details>
        </div>
    </div>
    <!-- CTA section -->
    <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-20 border-t border-gray-800">
        <div class="bg-linear-to-r from-blue-600/20 to-purple-600/20 border border-blue-500/30 rounded-xl p-12 text-center">
            <h2 class="text-white text-3xl font-bold mb-4">Ready to explore QuestSLAM?</h2>
            <p class="text-gray-400 mb-8 max-w-2xl mx-auto">
                Start building with open-source SLAM today. No proprietary lock-in, just pure robotics innovation.
            </p>
            <div class="flex flex-col sm:flex-row gap-4 justify-center">
                <a href="https://github.com/KirvilNET/QuestSLAM" class="px-8 py-3 text-white bg-blue-600 hover:bg-blue-700 rounded-lg font-semibold transition-colors">
                    View on GitHub
                </a>
                <a href="#" class="text-white px-8 py-3 border border-gray-600 hover:border-gray-400 rounded-lg font-semibold transition-colors">
                    Read Docs
                </a>
            </div>
        </div>
    </div>
	</div>
</template>
