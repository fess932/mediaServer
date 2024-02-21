<script setup lang="ts">
import { ref } from 'vue';

interface Image {
  name: String,
  path: String,
  type: String
}

const images = ref<Image[]>([]);

let update = async () => {
  images.value = []
  try {
    let resp = await fetch("http://localhost:3100");
    let json = await resp.json()

    json.forEach((element: Image) => {
      if (element.type ===  "dir") {
        return
      }
      if (!element.path.endsWith(".jpg")) {
        return
      }

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
  <div v-for="index in 10" :key="index" class="img">
    <a :href="`http://localhost:3100/static/${images[index].path}`"></a>
    <img :src="`http://localhost:3100/static/${images[index].path}`"  alt=""/>
  </div>

<!--  <div v-for="image in images" class="img">-->
<!--    <a :href="`http://localhost:3100/static/${image.path}`"></a>-->
<!--    <img :src="`http://localhost:3100/static/${image.path}`"  alt=""/>-->
<!--  </div>-->
</template>

<style scoped>
.img {
  width: 300px;
  overflow: hidden;
}
</style>
