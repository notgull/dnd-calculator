<--
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
  <div class="home">
    <h1>D&D Calculation Site</h1>
    <table>
      <DiceRoller v-for="(dice_type, index) in dice_types" :dice_type="dice_type" :key="index" />
    </table>
  </div>
</template>

<script>
import DiceRoller from "@/components/DiceRoller.vue";
import Vue from "vue";
import Component from "vue-class-component";

@Component({
  components: {
    DiceRoller
  }
})
export default class Home extends Vue {
  dice_types = [2, 4, 6, 8, 10, 12, 20, 100]
}
</script>
