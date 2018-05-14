<script>
export default {
  computed: {
    username() {
      return this.$route.params.username;
    }
  },
  methods: {
    goback() {
      window.history.length > 1
      ? this.$route.go(-1)
      : this.$route.push('/')
    }
  }
}
</script>