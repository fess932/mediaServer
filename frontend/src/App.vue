<script setup lang="ts">
import {ref} from 'vue';

interface File {
  name: String,
  path: String,
  type: String
}

const images = ref<File[]>([])
const dirs = ref<File[]>([])

let update = async () => {
  images.value = []
  try {
    let resp = await fetch("http://localhost:3100");
    let json = await resp.json()

    json.forEach((element: File) => {
      if (element.type === "dir") {
        dirs.value.push(element)
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

  <div class="dirs">
    <div v-for="dir in dirs" class="dir">
      <h5>dir {{ dir.name }}</h5>
      <!--      <a :href="`http://localhost:3100/static/${dir.path}`"></a>-->
      <!--    <img :src="`http://localhost:3100/static/${images[index].path}`"  alt=""/>-->
    </div>
  </div>


  <div class="imgs">
    <!--    <div v-for="index in 10" :key="index" class="img">-->
    <!--      <a :href="`http://localhost:3100/static/${images[index].path}`"></a>-->
    <!--      <img :src="`http://localhost:3100/static/${images[index].path}`" loading="lazy" alt=""/>-->
    <!--    </div>-->

    <div v-for="image in images" class="img">
      <a :href="`http://localhost:3100/static/${image.path}`"></a>
      <img :src="`http://localhost:3100/static/${image.path}`" loading="lazy" class="img-elem" alt=""/>
    </div>
  </div>


</template>

<style scoped>
.dirs {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
}

.dir {
  margin: 5px;
  border-radius: 3%;
  width: 300px;
  height: 300px;
  background: #5b97b6;
}

.imgs {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
}

.img {
  margin: 5px;
  border-radius: 3%;
  width: 300px;
  height: 500px;
  background: #5b97b6;
  overflow: hidden;
}

.img-elem {
  width: 400px;
  height: 600px;
  object-fit: cover;
}

</style>
