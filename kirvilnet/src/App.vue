<script setup lang="ts">
  import { ref } from 'vue'
  import nav_navbar from "../src/components/nav/navbar.vue"
  import nav_footer from "../src/components/nav/footer.vue"

  const needsNavBar = ref(true)
  const needsFooter = ref(true)
</script>

<template>
  <div class="flex-cols h-screen">
    <div h-95px>
      <template v-if="needsNavBar">
        <nav_navbar />
      </template>
    </div>
    <div>
      <router-view />
    </div>
    <div class="h-245px">
      <template v-if="needsFooter">
        <nav_footer />
      </template>
    </div>
  </div>
</template>
