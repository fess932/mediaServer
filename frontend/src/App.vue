<script setup lang="ts">
import { ref } from 'vue';

interface Image {
  name: String,
  path: String
}

const images = ref<Image[]>([]);

let update = async () => {
  images.value = []
  try {
    let resp = await fetch("http://localhost:4444");
    let json = await resp.json()

    json.forEach((element: any) => {
      console.log(element)
      images.value.push(element)
    });

    console.log(images)
  } catch (e) {
    console.error("wtf", e)
  }
}
update()

</script>

<template>
  <button @click="update">update</button>
  <div>
    images
  </div>
  <div v-for="image in images">
    <a :href="`http://localhost:4444/static/${image.path}`"></a>
    <img :src="`http://localhost:4444/static/${image.path}`" />
  </div>
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
