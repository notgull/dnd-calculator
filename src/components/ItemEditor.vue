<!--
  Copyright 2020 not_a_seagull

  Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated
  documentation files (the "Software"), to deal in the Software without restriction, including without limitation
  the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software,
  and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

  The above copyright notice and this permission notice shall be included in all copies or substantial portions
  of the Software.

  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
  TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
  THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
  CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
  DEALINGS IN THE SOFTWARE.
 -->

<template>
  <Modal :width="'33%'" :z_index="13">
    <h2 slot="header">Edit Item</h2>
    <div slot="body">
      <p><b>Name: </b><input type="text" v-model="item_data.name" /></p>
      <p><b>Description: </b><input type="text" v-model="item_data.description" /></p>
    </div>
    <div slot="footer">
      <input type="button" value="Cancel" @click="$emit('close')" />
      <input type="button" value="Submit" :disabled="!validity()" @click="submit" />
    </div>
  </Modal>
</template>

<script lang="ts">
import axios from "axios";
import Modal from "./Modal.vue";
import Vue from "vue";
import Component from "vue-class-component";
import { Item } from "@/types";
import { Prop, Watch } from "vue-property-decorator";

@Component({
  components: { Modal }
})
export default class ItemEditor extends Vue {
  @Prop({ type: Number, default: -1 }) item_id!: number;
  @Prop({ type: Object, default: null }) item!: Item | null;

  item_data: Item = { name: "", description: "" };
  processing: boolean = false;

  @Watch("item", { deep: true, immediate: true })
  update_item(item: Item) {
    if (item) {
      this.item_data = item;
    }
  }

  validity(): boolean {
    return this.item_data.name.length > 0 &&
           this.item_data.description.length > 0 &&
           !this.processing;
  }

  async submit() {
    this.processing = true;
    try {
      // @ts-ignore
      if (parseInt(this.item_id, 10) === -1) {
        await axios.post("/api/create_item", item);
      } else {
        await axios.post("/api/edit_item", { item_id: this.item_id, item });
      } 
    } finally {
      this.processing = false;
    }

    this.$emit("submitted");
  }
}
</script>
