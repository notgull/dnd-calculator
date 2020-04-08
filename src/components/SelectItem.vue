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
  <div class="item-selector">
    <p v-if="loading">Loading...</p>
    <p><input type="text" v-model="search" :disabled="loading" /></p>
    <ul>
      <li v-for="(item, index) in displayed_items()" :key="index" :style="get_li_style(index)">
        <p class="item-bar" @click="select_item(item, index)">
          {{ item.item.name }} - {{ truncate_item_desc(item) }} - 
          <input type="button" value="Edit" :disabled="loading" @click="edit_item(item)" />
        </p>
      </li>
    </ul>
  </div>
  <ItemEditor v-if="editing_item" 
              :item="edit_item" 
              :item_id="edit_item_id" 
              @close="editing_item = false"
              @submitted="on_edit_item" />
</template>

<script lang="ts">
import axios from "axios";
import Vue from "vue";
import Component from "vue-class-component";
import { Prop } from "vue-property-decorator";
import { Item, SelectItemFunction } from "@/types";

import ItemEditor from "./ItemEditor.vue";

const truncate_len = 20;

export interface ItemDescription {
  item_id: number,
  item: Item,
};

@Component({
  components: { ItemEditor },
  async created(this: SelectItem): Promise<void> {
    return this.load_items();
  }
})
export class SelectItem extends Vue {
  loading: bool = false;
  items: ItemDescription[] = [];
  search: string = "";
  edit_item: Item | null = null; 
  edit_item_id: number = -1;
  editing_item: boolean = false;
  selected_index: number = -1;

  get_li_style(index: number): string {
    if (index === selected_index) {
      return "background-color: green; color: white";
    } else {
      return "";
    }
  }

  displayed_items(): ItemDescription[] {
    return items.filter((i) => i.item.name.indexOf(search) !== -1);
  }

  truncate_item_desc(item: ItemDescriptor): string {
    let res = item.item.description.substring(0, truncate_len);
    if (res !== item.item.description) {
      res += "...";
    }
    return res;
  }

  async load_items(): Promise<void> {
    this.loading = true;
   
    try {
      this.items = (await axios.post("/api/list_items", {})).filter((i) => {
        return {
          item_id: i.item_id,
          id: {
            name: i.name,
            description: i.description,
          },
        };
      });
    } finally {
      this.loading = false;
    }
  }

  edit_item(item: ItemDescriptor) {
    this.edit_item = item.item;
    this.edit_item_id = item.item_id;
    this.editing_item = true; 
  }

  create_item() {
    this.edit_item = null;
    this.edit_item_id = -1;
    this.editing_item = true;
  }

  select_item(item: ItemDescriptor, index: number) {
    this.selected_index = index;
    this.$emit("selected_item", item);
  }

  async on_edit_item(): Promise<void> {
    this.editing_item = false;
    return this.load_items();
  }
}; 
</script>

<style scoped lang="scss">
.item-selector {
  border: 1px solid black;
  overflow: scroll;
  max-height: 300px;

  .item-bar {
    border-bottom: 1px solid dotted;
  }

  .item-bar.hover {
    background-color: blue;
    color: white;
  }
}
</style>
