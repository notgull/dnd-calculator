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
  <Modal :width="'50%'">
    <h2 slot="header">Edit Template</h2>
    <div slot="body">
      <p><b>Name: </b><input type="text" v-model="name" @change="name_changed = true" /></p>
      <p><b>Description: </b><input type="text" v-model="description" @change="desc_changed = true" /></p>
      <p><b>Health: </b><input type="number" v-model="health" @change="h_changed = true" /></p>
      <p><b>Armor Class: </b><input type="number" v-model="armor_class" @change="ac_changed = true" /></p>
    </div>
    <div slot="footer">
      <input type="button" value="Submit" :disabled="!validity()" @click="submit" />
      <input type="button" value="Cancel" @click="$emit('close')" />
    </div>
  </Modal>
</template>

<script lang="ts">
import axios from "axios";
import Modal from "./Modal.vue";
import Vue from "vue";
import Component from "vue-class-component";
import { Prop } from "vue-property-decorator";
import { DeleteFunction, Template } from "@/types";

@Component({
  components: { Modal },
})
export default class TemplateEditor extends Vue {
  @Prop() is_new_template!: boolean;
  @Prop() init_template_id!: number;
  @Prop() init_template_data!: Template;

  _new_template: boolean | null = null;
  _template_id: number | null = null;

  name: String = "";
  name_changed: boolean = false;

  description: String = "";
  desc_changed: boolean = false;

  health: number = 1;
  h_changed: boolean = false;

  armor_class: number = 10;
  ac_changed: boolean = false;
  
  submitting: boolean = false;

  validity() {
    return this.name.length > 0 &&
           this.description.length > 0 &&
           this.health > 0 &&
           this.armor_class > 0 &&
           !this.submitting;
  }

  get new_template(): boolean {
    if (this._new_template === null || this._new_template === undefined) {
      this._new_template = this.is_new_template;
    }
    return this._new_template;
  }

  set new_template(val: boolean) {
    this._new_template = val;
  }

  get template_id(): number {
    if (this._template_id === null || this._template_id === undefined) {
      this._template_id = this.init_template_id;
    }
    return this._template_id;
  }

  set template_id(val: number) {
    this._template_id = val;
  }

  async submit() {
    let template = {
      name: this.name, 
      // @ts-ignore
      health: parseInt(this.health, 10),
      // @ts-ignore
      armor_class: parseInt(this.armor_class, 10),
      description: this.description, 
    };  
    
    console.log("Is this a new template? ", this.new_template);
    console.log("Is this a new template?? ", this.is_new_template);
    console.log("Template: ", JSON.stringify(template));

    this.submitting = true;
    try {
      if (this.new_template) {
        let { res } = await axios.post("/api/create_template", template);
        console.log(res); // TODO: not this
      } else {
        await axios.post("/api/edit_template", { template, tid: this.template_id });
      }
    } finally {
      this.submitting = false;
    }

    this.$emit("submitted");
  }
};
</script>
