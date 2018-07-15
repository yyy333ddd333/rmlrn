<template>
  <div id="app">
    <img src="./assets/logo.png">
    <HelloWorld msg="Welcome to Your Vue.js App"/>
    <input v-model="shareNo" />
    <button @click="getInfo">查询</button>
  </div>
</template>

<script>
import HelloWorld from './components/HelloWorld.vue'
import axios from 'axios'
// import jsonp from 'jsonp'

export default {
  name: 'app',
  data: function(){
    return {
      shareNo: "sh601009",
    }
  },
  methods: {
    getInfo() {
      axios.get('http://localhost:3000/api/stock/sh000001')
      .then(res => console.log(res))
      // jsonp('http://web.juhe.cn:8080/finance/stock/hs?gid=sh601009&key=b8b2dbe7bebf93ed84842bce7713f452')
    }
  },
  components: {
    HelloWorld
  }
}
</script>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
