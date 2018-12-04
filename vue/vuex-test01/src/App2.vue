<template>
  <div id='app2'>
    <button type="button" v-on:click="inc">+</button>  
  </div>
</template>

<script>
//import store from './store.js'
export default{
  methods: {
    inc: function(){console.log('inc')}
  }  
}
</script>
