import VConsole from "vconsole";
new VConsole();

import Vue from 'vue'
import VueRouter from 'vue-router'

import App1 from './App1.vue'

Vue.config.productionTip = false

Vue.use(VueRouter)
/*
new Vue({
  render: h => h(App)
}).$mount('#app')
*/

import Foo from './Foo.vue'
import Bar from './Bar.vue'
//const Foo = { template: '<div>foo</div>' }
//const Bar = { template: '<div>bar</div>' }
const routes = [
  { path: '/foo', component: Foo },
  { path: '/bar', component: Bar }
]
const router = new VueRouter({
  routes // （缩写）相当于 routes: routes
})
new Vue({
  router,
  render: h => h(App1)
}).$mount('#app1')

import User from './User.vue'
const router2 = new VueRouter({
  routes: [
    {path: '/user/:id', component: User}
  ]
})

import App2 from './App2.vue'

new Vue({
  router: router2,
  render: h => h(App2)
}).$mount('#app2')

import User2 from './User2.vue'
import UserProfile from './UserProfile.vue'
import UserPosts from './UserPosts.vue'
import UserHome from './UserHome.vue'

const router3 = new VueRouter({
  routes: [
    {
      path: '/user/:id',
      component: User2,
      children: [
        {
          path: '',
          component: UserHome
        },
        {
          path: 'profile',
          component: UserProfile
        },
        {
          path: 'posts',
          component: UserPosts
        }
      ]
    }
  ]
})

import App3 from './App3.vue'

new Vue({
  router: router3,
  render: h => h(App3)
}).$mount('#app3')
