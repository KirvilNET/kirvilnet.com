<script setup lang="ts">

</script>

<template>
  <div className="h-[calc(100vh-239px-95px)] mx-auto grid place-items-center text-center px-8 bg-black ">
    <img src="/media/topography.svg" class="absolute inset-0 h-full w-full object-cover"></img>
    <div class="flex flex-col">
      <div class="backdrop-blur p-8 rounded-2xl">
        <div class="flex justify-center mb-2 shrink-0">
          <svg class="fill-[#fc0303]" xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 512 512"><path d="M256 512a256 256 0 1 1 0-512 256 256 0 1 1 0 512zm0-192a32 32 0 1 0 0 64 32 32 0 1 0 0-64zm0-192c-18.2 0-32.7 15.5-31.4 33.7l7.4 104c.9 12.6 11.4 22.3 23.9 22.3 12.6 0 23-9.7 23.9-22.3l7.4-104c1.3-18.2-13.1-33.7-31.4-33.7z"/></svg>
        </div>
        <h1 class="text-6xl text-white">Uh Oh</h1>
        <p class="text-xl text-white">Error (404)</p>
        <router-link class="mt-5 mb-5 inline-flex justify-center items-center gap-x-3 text-center bg-linear-to-tl from-blue-600 to-violet-600 shadow-lg shadow-transparent hover:shadow-blue-700/50 border border-transparent text-white text-sm font-medium rounded-full focus:outline-hidden focus:shadow-blue-700/50 py-3 px-6" to="/">
          Back to Home
        </router-link>
        <p class="text-l text-white">If you think this is an error report it</p>
      </div>
    </div>
  </div>
</template>

