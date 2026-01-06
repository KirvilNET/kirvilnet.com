import { createRouter, createWebHistory } from 'vue-router'

// Main Pages
import home from "../pages/home.vue"

// Product Pages
import product_questslam from "../pages/products/questslam.vue"

// Error Pages
import NotFound from '../pages/NotFound.vue'

const routes = [
  { path: '/', name: 'Home', component: home},
  { path: '/products/questslam', name: 'product-questslam', component: product_questslam},
  { path: '/:pathMatch(.*)*', name: 'NotFound', component: NotFound }
]


const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

export default router
