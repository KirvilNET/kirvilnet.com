<script setup lang="ts"></script>

<template>
  <footer class="relative bg-[#121212] mt-auto z-30 w-full h-full py-10 px-4 sm:px-6 lg:px-8 mx-auto">
    <div class="w-full h-full max-w-340 mx-auto">
      <!-- Grid -->
      <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-5 gap-6 mb-10">
        <div class="col-span-full hidden lg:col-span-1 lg:block">
          <router-link class="flex-none font-semibold text-xl focus:outline-hidden focus:opacity-80 text-white" to="/" aria-label="Brand">
            <img src="/media/branding/full_logo.svg" width="150" height="75"></img>
          </router-link>
          <p class="mt-3 text-xs sm:text-sm  text-neutral-400">© 2026 KirvilNET. All rights reserved.</p>
        </div>
        <!-- End Col -->
        <div>
          <h4 class="text-xs font-semibold uppercase text-neutral-100">Products</h4>
          <div class="mt-3 grid space-y-3 text-sm">
            <p>
              <a class="inline-flex gap-x-2 text-gray-400 hover:text-gray-800 focus:outline-hidden focus:text-neutral-200" href="/products/questslam">QuestSLAM</a>
            </p>
          </div>
        </div>
        <!-- End Col -->
        <div>
          <h4 class="text-xs font-semibold  uppercase text-neutral-100">Company</h4>
          <div class="mt-3 grid space-y-3 text-sm">
            <p class="hidden">
              <a class="inline-flex gap-x-2 text-gray-400 hover:text-gray-800 focus:outline-hidden focus:text-neutral-200" href="/about-us">About us</a>
            </p>
            <p>
              <a class="inline-flex gap-x-2 text-gray-400 hover:text-gray-800 focus:outline-hidden focus:text-neutral-200" href="https://services.kirvilnet.com/">Services</a>
            </p>
          </div>
        </div>
        <!-- End Col -->
        <div>
          <h4 class="text-xs font-semibold  uppercase text-neutral-100">Developer</h4>
          <div class="mt-3 grid space-y-3 text-sm">
            <p>
              <a class="inline-flex gap-x-2 text-gray-400 hover:text-gray-800 focus:outline-hidden focus:text-neutral-200" href="https://github.com/KirvilNET/">Github</a>
            </p>
            <p>
              <a class="inline-flex gap-x-2 text-gray-400 hover:text-gray-800 focus:outline-hidden focus:text-neutral-200" href="/docs">Docs</a>
            </p>
          </div>
        </div>
        <!-- End Col -->
      </div>
      <!-- End Grid -->
      <div class="pt-5 mt-5 border-t border-violet-600">
        <div class="sm:flex sm:justify-between sm:items-center">
          <div class="flex flex-wrap items-center gap-3">
            <div class="hidden space-x-4 text-sm">
              <a class="inline-flex gap-x-2 text-neutral-400 hover:text-gray-800 focus:outline-hidden focus:text-neutral-200" href="#">Terms</a>
              <a class="inline-flex gap-x-2 text-neutral-400 hover:text-gray-800 focus:outline-hidden focus:text-neutral-200" href="#">Privacy</a>
            </div>
          </div>
          <div class="flex flex-wrap justify-between items-center gap-3">
            <div class="mt-3 sm:hidden">
              <router-link class="flex-none font-semibold text-xl focus:outline-hidden focus:opacity-80 text-white" to="/" aria-label="Brand">
                <img src="/media/branding/full_logo.svg" width="150" height="75"></img>
              </router-link>
              <p class="mt-1 text-xs sm:text-sm text-gray-400">© 2026 KirvilNET. All rights reserved.</p>
            </div>
          </div>
          <!-- End Col -->
        </div>
      </div>
    </div>
  </footer>
</template>
