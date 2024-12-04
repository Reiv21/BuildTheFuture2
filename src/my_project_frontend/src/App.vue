<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
let greeting = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const title = target.querySelector('#title').value;
  const content = target.querySelector('#content').value;
  const tags = target.querySelector('#tags').value;

  const splittedTags = tags.split(",");
  await my_project_backend.add_blog(title, content, splittedTags);
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <div><p>Title: </p><input id="title" alt="title" type="text" /></div>
      
     <div><p>Content: </p><input id="content" alt="content" type="text" /></div>

     <div><p>Tags: </p><input id="tags" alt="tags" type="text" /></div>

      <button type="submit">Click to add blog UwU</button>
    </form>
  </main>
</template>
