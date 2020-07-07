import Vue from 'vue'
import axios from 'axios'

Vue.prototype.$getArticle = async (articleName) => {
  return await axios
    .get(`/api/articles/${articleName}`)
    .catch((error) => {
      return error
    })
    .then((response) => {
      if (response.data) return response.data
      return {
        status: 'error',
        content: 'No data found from response'
      }
    })
}

Vue.prototype.$getAllArticles = async () => {
  return await axios
    .get('/api/articles/')
    .catch((error) => {
      return error
    })
    .then((response) => {
      if (response.data) return response.data
      return {
        status: 'error',
        content: 'No data found from response'
      }
    })
}
