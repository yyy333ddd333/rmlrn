<template>
  <div>
    {{count}}{{count2}}{{count3}}
    <button @click="inc">+</button>  
  </div>
</template>

<script>
import { mapState } from 'vuex'

export default{
  methods: {
    inc: function(){
      //this.count++
      this.$store.commit("increment")
      console.log('inc')
    }
  },
  computed: {
    count() {
      return this.$store.state.count
    },
    ...mapState({
      count2: "count",
      count3: state => state.count
    })
  },
/*
  data(){
    return{
      count: 0
    }
  }
*/
}
</script>
