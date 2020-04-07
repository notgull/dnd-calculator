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
      <p><b>Name: </b><input type="text" v-model="template_data.name" @change="prop_change('name')" /></p>
      <p><b>Description: </b><input type="text" v-model="template_data.description" @change="prop_change('desc')" /></p>
      <p><b>Health: </b><input type="number" v-model="template_data.health" @change="prop_change('health')" /></p>
      <p><b>Armor Class: </b><input type="number" v-model="template_data.armor_class" @change="prop_change('ac')" /></p>
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
import { Prop, Watch } from "vue-property-decorator";
import { DeleteFunction, Template } from "@/types";

@Component({
  components: { Modal }
})
export default class TemplateEditor extends Vue {
  @Prop({ type: Number, default: -1 }) template_id!: number;
  @Prop({ type: Object, default: null }) template!: Template | null;

  changed_props: string[] = [];

  prop_change(val: string) {
    if (this.changed_props.indexOf(val) === -1) {
      this.changed_props.push(val);
    }
  }

  template_data: Template = {
    name: "",
    health: 0,
    armor_class: 0,
    description: "",
  }; 

  @Watch("template", { deep: true, immediate: true })
  update_template(tmpl: Template) {
    console.log("Template updated!");
    if (tmpl) {
      this.template_data = tmpl;
    }
  }

  get mdl(): Template { return this.template_data; }
  
  submitting: boolean = false;

  validity() {
    return this.mdl.name.length > 0 &&
           this.mdl.description.length > 0 &&
           this.mdl.health > 0 &&
           this.mdl.armor_class > 0 &&
           !this.submitting;
  }

  async submit() {
    let template = {
      name: this.mdl ? this.mdl.name : "",
      // @ts-ignore
      health: parseInt(this.template_data.health, 10),
      // @ts-ignore
      armor_class: parseInt(this.template_data.armor_class, 10),
      description: this.mdl  ? this.mdl.description : "", 
    };

    this.submitting = true;
    try {
      // @ts-ignore
      if (parseInt(this.template_id, -1) !== -1) {
        await axios.post("/api/create_template", template);
      } else {
        await axios.post("/api/edit_template", { tid: this.template_id, template });
      }
    } finally {
      this.submitting = false;
    }

    this.$emit("submitted");
  }
};
</script>
