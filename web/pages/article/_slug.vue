<template>
  <div class="app">
    <section class="container">
      <div class="card">
        <div v-if="article.haveImage" class="card-image">
          <figure class="image is-4by3">
            <img
              :src="`http://localhost:8000/api/article/image/${article.title}`"
              alt="Placeholder image"
            />
          </figure>
        </div>
        <div class="card-content has-text-centered">
          <p v-if="article.title" class="title is-4 is-capitalized">
            {{ article.title.replace(/-/g, ' ') }}
          </p>
          <p class="subtitle is-6">{{ article.occurredAt }}</p>
          <p class="subtitle is-6">{{ article.mediaSource }}</p>

          <footer class="card-footer">
            <a href="#" class="card-footer-item">
              新闻标签:
              {{ article.eventTagList }}
            </a>
            <a href="#" class="card-footer-item">
              板块标签:
              {{ article.sectionTagList }}
            </a>
            <a href="#" class="card-footer-item">
              相关企业:
              {{ article.stakeholderList }}
            </a>
          </footer>
          <p class="content">
            <markduck
              v-if="article.content"
              :markdown="article.content"
            ></markduck>
          </p>
        </div>
      </div>
    </section>
  </div>
</template>

<script>
import markduck from 'markduckjs'

export default {
  components: {
    markduck: markduck({})
  },
  data() {
    return {
      article: []
    }
  },
  mounted() {
    const articleParam = this.$route.params.slug

    this.$getArticle(articleParam).then((response) => {
      if (response.status === 'error') {
        return this.$nuxt.error({ statusCode: 404, message: response.content })
      } else {
        this.article = response
        console.log(this.article)
      }
    })
  },
  head() {
    return {
      title: 'testapp'
    }
  }
}
</script>
