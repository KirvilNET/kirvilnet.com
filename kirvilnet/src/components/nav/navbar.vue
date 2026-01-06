<script setup lang="ts">
  import '@preline/collapse'
  import '@preline/dropdown'
</script>

<template>
  <!-- ========== HEADER ========== -->
  <header class="flex flex-wrap relative md:justify-start md:flex-nowrap z-30 w-full h-full border-b border-violet-600 bg-[#121212]">
    <nav class="relative max-w-340 w-full mx-auto md:flex md:items-center md:justify-between md:gap-3 py-2 px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between items-center gap-x-1">
        <router-link class="flex-none focus:opacity-80" to="/" aria-label="logo">
          <img src="/media/branding/full_logo.svg" width="240" height="100"></img>
        </router-link>
        <!-- Collapse Button -->
        <button type="button" class="hs-collapse-toggle md:hidden relative size-9 flex justify-center items-center font-medium text-sm rounded-lg border focus:outline-hidden disabled:opacity-50 disabled:pointer-events-none text-white border-neutral-700 hover:bg-neutral-700 focus:bg-neutral-700" id="hs-header-base-collapse" aria-expanded="false" aria-controls="hs-header-base" aria-label="Toggle navigation" data-hs-collapse="#hs-header-base">
          <svg class="hs-collapse-open:hidden size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="3" x2="21" y1="6" y2="6" />
            <line x1="3" x2="21" y1="12" y2="12" />
            <line x1="3" x2="21" y1="18" y2="18" />
          </svg>
          <svg class="hs-collapse-open:block shrink-0 hidden size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M18 6 6 18" />
            <path d="m6 6 12 12" />
          </svg>
          <span class="sr-only">Toggle navigation</span>
        </button>
        <!-- End Collapse Button -->
      </div>
      <!-- Collapse -->
      <div id="hs-header-base" class="hs-collapse hidden overflow-hidden transition-all duration-300 basis-full grow md:block" aria-labelledby="hs-header-base-collapse">
        <div class="overflow-hidden overflow-y-auto max-h-[75vh] [&::-webkit-scrollbar]:w-2 [&::-webkit-scrollbar-thumb]:rounded-full [&::-webkit-scrollbar-track]:bg-neutral-700 [&::-webkit-scrollbar-thumb]:bg-neutral-500">
          <div class="py-2 md:py-0 flex flex-col md:flex-row md:items-center gap-0.5 md:gap-1">
            <div class="grow">
              <div class="flex flex-col md:flex-row md:justify-end md:items-center gap-0.5 md:gap-1">
                <!-- Mega Menu -->
                <div class="hs-dropdown [--strategy:static] md:[--strategy:fixed] [--adaptive:none] md:[--adaptive:adaptive] [--is-collapse:true] md:[--is-collapse:false]">
                  <button id="hs-dropdown" type="button" class="hs-dropdown-toggle w-full p-2 flex items-center text-sm  rounded-lg focus:outline-hidden  text-neutral-200 hover:bg-neutral-700 focus:bg-neutral-700" aria-haspopup="menu" aria-expanded="false" aria-label="Mega Menu">
                    <svg class="shrink-0 size-4 me-3 md:me-2 block md:hidden fill-[#8c52ff]" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" />
                      <circle cx="12" cy="12" r="3" />
                    </svg>
                    Products
                    <svg class="hs-dropdown-open:-rotate-180 md:hs-dropdown-open:rotate-0 duration-300 shrink-0 size-4 ms-auto md:ms-1" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6" /></svg>
                  </button>
                  <div class="hidden hs-dropdown-menu transition-[opacity,margin] duration md:duration-150 hs-dropdown-open:opacity-100 relative md:w-80 z-20 top-full md:rounded-lg md:shadow-md before:absolute before:-top-4 before:start-0 before:w-full before:h-5 md:bg-neutral-800" role="menu" aria-orientation="vertical" aria-labelledby="product-dropdown">
                    <div class="py-1 md:px-1 space-y-0.5">
                      <!-- Link -->
                      <a class="p-3 flex gap-x-4  focus:outline-hidden rounded-lg text-neutral-200 hover:bg-neutral-700 focus:bg-neutral-700" href="/products/questslam">
                        <svg class="fill-[#8c52ff]" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512" width="24" height="24"><path d="M512 96L64 96C28.7 96 0 124.7 0 160L0 352c0 35.3 28.7 64 64 64l117.5 0c17 0 33.3-6.7 45.3-18.7l33.9-33.9c7.2-7.2 17.1-11.3 27.3-11.3s20.1 4.1 27.3 11.3l33.9 33.9c12 12 28.3 18.7 45.3 18.7L512 416c35.3 0 64-28.7 64-64l0-192c0-35.3-28.7-64-64-64zM80 240a64 64 0 1 1 128 0 64 64 0 1 1 -128 0zm352-64a64 64 0 1 1 0 128 64 64 0 1 1 0-128z" /></svg>
                        <div class="grow">
                          <span class="block font-semibold text-sm text-neutral-200">QuestSLAM</span>
                          <p class="text-sm text-neutral-500">A SLAM implemtation for ROS2 using the Meta Quest3 and unity</p>
                        </div>
                      </a>
                      <!-- End Link -->
                      <div class="hidden my-2 border-t border-gray-100"></div>
                      <!-- Link -->
                      <a class="hidden p-3 flex gap-x-4 focus:outline-hidden rounded-lg text-neutral-200 hover:bg-neutral-700 focus:bg-neutral-700" href="/products/lattice">
                        <svg class="fill-[#8c52ff]" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" width="24" height="24"><path d="M448 205.8c-14.8 9.8-31.8 17.7-49.5 24-47 16.8-108.7 26.2-174.5 26.2S96.4 246.5 49.5 229.8c-17.6-6.3-34.7-14.2-49.5-24L0 288c0 44.2 100.3 80 224 80s224-35.8 224-80l0-82.2zm0-77.8l0-48C448 35.8 347.7 0 224 0S0 35.8 0 80l0 48c0 44.2 100.3 80 224 80s224-35.8 224-80zM398.5 389.8C351.6 406.5 289.9 416 224 416S96.4 406.5 49.5 389.8c-17.6-6.3-34.7-14.2-49.5-24L0 432c0 44.2 100.3 80 224 80s224-35.8 224-80l0-66.2c-14.8 9.8-31.8 17.7-49.5 24z" /></svg>
                        <div class="grow">
                          <span class="block font-semibold text-sm text-neutral-200">Router</span>
                          <p class="text-sm text-gray-500">Reverse-proxy manager and service discovery</p>
                        </div>
                      </a>
                      <!-- End Link -->
                    </div>
                  </div>
                </div>
                <!-- End Mega Menu -->
                <a class="p-2 flex items-center text-sm rounded-lg focus:outline-hidden text-neutral-200 hover:bg-neutral-700 focus:bg-neutral-700" href="https://docs.kirvilnet.com">
                  <svg class="shrink-0 size-4 me-3 md:me-2 block md:hidden fill-[#8c52ff]" width="24" height="24" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640"><path d="M480 576L192 576C139 576 96 533 96 480L96 160C96 107 139 64 192 64L496 64C522.5 64 544 85.5 544 112L544 400C544 420.9 530.6 438.7 512 445.3L512 512C529.7 512 544 526.3 544 544C544 561.7 529.7 576 512 576L480 576zM192 448C174.3 448 160 462.3 160 480C160 497.7 174.3 512 192 512L448 512L448 448L192 448zM224 216C224 229.3 234.7 240 248 240L424 240C437.3 240 448 229.3 448 216C448 202.7 437.3 192 424 192L248 192C234.7 192 224 202.7 224 216zM248 288C234.7 288 224 298.7 224 312C224 325.3 234.7 336 248 336L424 336C437.3 336 448 325.3 448 312C448 298.7 437.3 288 424 288L248 288z" /></svg>
                  Docs
                </a>
              </div>
            </div>
          </div>
        </div>
      </div>
      <!-- End Collapse -->
    </nav>
  </header>
</template>
