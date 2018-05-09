import Vue from 'vue'
//import App from './App.vue'

import VueRouter from 'vue-router'

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
    router
}).$mount('#app1')
