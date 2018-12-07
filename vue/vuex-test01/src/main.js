import VConsole from 'vconsole'
var vConsole = new VConsole()

import Vue from 'vue'
//import App from './App.vue'
import App2 from './App2.vue'
import store from './store'

Vue.config.productionTip = false
/*
new Vue({
  store,
  render: h => h(App)
}).$mount('#app')
*/
new Vue({
  store,
  render: h => h(App2)
}).$mount('#app')
//store.commit('increment')
console.log(store.getters.doneTodos, store.getters.doneTodosCount)
