<template>
  <b-navbar type="is-black" wrapper-class="container">
    <template slot="brand">
      <b-navbar-item tag="router-link" :to="{ path: '/' }">
        <img src="~assets/images/logo.png" alt="To-do List" />
      </b-navbar-item>
    </template>

    <template slot="start">
      <b-navbar-item href="/articles">
        <span>
          舆情
        </span>
      </b-navbar-item>
      <b-navbar-item href="/markets">
        <span>
          行情
        </span>
      </b-navbar-item>
    </template>

    <template slot="end">
      <b-navbar-item href="#">
        <span>
          <b-icon icon="email"></b-icon>
        </span>
      </b-navbar-item>
    </template>
  </b-navbar>
</template>

<script>
export default {
  name: 'SiteNavbar'
}
</script>
