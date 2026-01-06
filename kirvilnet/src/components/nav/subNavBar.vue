<script setup lang="ts">
  import type { PageSections } from '../Objects';

  const props = defineProps<{
    ProductTitle: string
    ProductCatagory: string
    pageSections: PageSections[]
  }>()

  function scrollToSection(section: string) {
    document.getElementById(section)?.scrollIntoView({ behavior: 'smooth' });
  }
</script>

<template>
  <!-- Nav -->
	<nav class="h-32px sticky z-20 top-0 bg-[#8c52ff]">
		<div class="max-w-340 w-full mx-auto sm:flex sm:flex-row sm:justify-between sm:items-center sm:gap-x-3 py-3 px-4 sm:px-6 lg:px-8">
			<div class="flex justify-between items-center gap-x-3">
				<div class="grow gap-x-2 flex flex-row">
					<button class="font-semibold whitespace-nowrap text-white" @click="scrollToSection('hero')">{{ ProductTitle }}</button>
          <span class="py-1 px-1 border border-blue-500/50 bg-blue-500/50 rounded text-xs text-white">{{ ProductCatagory }}</span>
				</div>

				<button type="button" class="hs-collapse-toggle sm:hidden py-1.5 px-2 inline-flex items-center font-medium text-xs rounded-md border border-gray-200 bg-white text-black-800  shadow-2xs hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-800 focus:outline-hidden focus:bg-gray-100 dark:border-neutral-700 dark:text-neutral-300 dark:hover:bg-neutral-700 dark:focus:bg-neutral-700" data-hs-collapse="#hs-nav-secondary" aria-controls="hs-nav-secondary" aria-label="Toggle navigation">
					Overview
					<svg class="hs-dropdown-open:rotate-180 shrink-0 size-4 ms-1" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>
				</button>

			</div>
			<div id="hs-nav-secondary" class="hs-collapse hidden overflow-hidden transition-all duration-300 basis-full grow sm:block">
				<div class="py-2 sm:py-0 flex flex-col sm:flex-row sm:justify-end gap-y-2 sm:gap-y-0 sm:gap-x-6">

          <div v-for="section in pageSections" >
            <button class="font-medium text-sm text-white  hover:text-[#dec5ff] focus:outline-hidden focus:text-[#dec5ff] " @click="scrollToSection(section.pageName)">{{section.buttonName}}</button>
          </div>

        </div>
			</div>
		</div>
	</nav>
	<!-- End Nav -->
</template>
