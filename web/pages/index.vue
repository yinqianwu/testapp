<template>
  <div class="app">
    <section>
      <div class="column has-text-centered">
        <h1 class="title">开发中...</h1>
      </div>
    </section>
    <section class="container">
      <div v-for="i in Math.ceil(articles.length / 3)" :key="i" class="columns">
        <div v-if="articles[(i - 1) * 3].title" class="column">
          <div class="card">
            <div v-if="articles[(i - 1) * 3].haveImage" class="card-image">
              <figure class="image is-4by3">
                <img
                  :src="
                    `
                    http://localhost:8000/api/article/image/${
                      articles[(i - 1) * 3].title
                    }
                    `
                  "
                  alt="Placeholder image"
                />
              </figure>
            </div>
            <div class="card-content has-text-centered">
              <p class="title is-4 is-capitalized">
                {{ articles[(i - 1) * 3].title.replace(/-/g, ' ') }}
              </p>
              <p class="subtitle is-6">
                {{ articles[(i - 1) * 3].occurredAt }}
              </p>
              <p>{{ removeMarkdown(articles[(i - 1) * 3].description) }}</p>
            </div>
            <footer class="card-footer">
              <a
                :href="`/article/${articles[(i - 1) * 3].slug}`"
                class="card-footer-item"
              >
                更多...
              </a>
            </footer>
          </div>
        </div>
        <div v-if="articles[(i - 1) * 3 + 1]" class="column">
          <div class="card">
            <div v-if="articles[(i - 1) * 3 + 1].haveImage" class="card-image">
              <figure class="image is-4by3">
                <img
                  :src="
                    `
                    http://localhost:8000/api/article/image/${
                      articles[(i - 1) * 3 + 1].title
                    }
                    `
                  "
                  alt="Placeholder image"
                />
              </figure>
            </div>
            <div class="card-content has-text-centered">
              <p class="title is-4 is-capitalized">
                {{ articles[(i - 1) * 3 + 1].title.replace(/-/g, ' ') }}
              </p>
              <p class="subtitle is-6">
                {{ articles[(i - 1) * 3 + 1].occurredAt }}
              </p>
              <p>{{ removeMarkdown(articles[(i - 1) * 3 + 1].description) }}</p>
            </div>
            <footer class="card-footer">
              <a
                :href="`/article/${articles[(i - 1) * 3 + 1].slug}`"
                class="card-footer-item"
              >
                更多...
              </a>
            </footer>
          </div>
        </div>
        <div v-if="articles[(i - 1) * 3 + 2]" class="column">
          <div class="card">
            <div v-if="articles[(i - 1) * 3 + 2].haveImage" class="card-image">
              <figure class="image is-4by3">
                <img
                  :src="
                    `
                    http://localhost:8000/api/article/image/${
                      articles[(i - 1) * 3 + 2].title
                    }
                    `
                  "
                  alt="Placeholder image"
                />
              </figure>
            </div>
            <div class="card-content has-text-centered">
              <p class="title is-4 is-capitalized">
                {{ articles[(i - 1) * 3 + 2].title.replace(/-/g, ' ') }}
              </p>
              <p class="subtitle is-6">
                {{ articles[(i - 1) * 3 + 2].occurredAt }}
              </p>
              <p>{{ removeMarkdown(articles[(i - 1) * 3 + 2].description) }}</p>
            </div>
            <footer class="card-footer">
              <a
                :href="`/article/${articles[(i - 1) * 3 + 2].slug}`"
                class="card-footer-item"
              >
                更多...
              </a>
            </footer>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script>
const removeMd = require('remove-markdown')

export default {
  data() {
    return {
      articles: []
    }
  },
  mounted() {
    this.$getAllArticles().then(async (response) => {
      if (response.articles) {
        for (const index in response.articles) {
          const element = response.articles[index]

          await this.$getArticle(element.slug).then((response) => {
            if (response.status !== 'error') {
              response.content = response.content.substring(0, 200) + '...'
              this.articles.push(response)
            }
          })
        }
      } else {
        return this.$nuxt.error({
          statusCode: 404,
          message: 'Could not search for articles'
        })
      }
    })
  },
  methods: {
    removeMarkdown(markdownCode) {
      return removeMd(markdownCode)
    }
  },
  head() {
    return {
      title: 'testapp'
    }
  }
}
</script>
