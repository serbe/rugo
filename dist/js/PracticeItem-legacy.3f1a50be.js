(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["PracticeItem"],{"0bec":function(t,e,c){"use strict";var a=c("c408"),i=c.n(a);i.a},c408:function(t,e,c){},cf4f:function(t,e,c){"use strict";c.r(e);var a=function(){var t=this,e=t.$createElement,c=t._self._c||e;return c("div",{staticClass:"container mw768"},[c("bulma-select",{attrs:{list:t.companys,"selected-item":t.practice.company,label:"Организация","item-name":"company",iconLeft:"building"},on:{select:t.onSelect}}),c("bulma-select",{attrs:{list:t.kinds,"selected-item":t.practice.kind,label:"Тип тренировки","item-name":"kind",iconLeft:"tag"},on:{select:t.onSelect}}),c("bulma-input",{attrs:{label:"",placeholder:"Тема тренировки",iconLeft:"tag"},model:{value:t.practice.topic,callback:function(e){t.$set(t.practice,"topic",e)},expression:"practice.topic"}}),c("bulma-date",{attrs:{label:"Дата проведения тренировки"},model:{value:t.practice.date_of_practice,callback:function(e){t.$set(t.practice,"date_of_practice",e)},expression:"practice.date_of_practice"}}),c("bulma-input",{attrs:{label:"Заметка",placeholder:"Заметка",iconLeft:"comment"},model:{value:t.practice.note,callback:function(e){t.$set(t.practice,"note",e)},expression:"practice.note"}}),c("div",{staticClass:"field is-grouped is-grouped-centered"},[c("div",{staticClass:"control"},[c("bulma-button",{attrs:{text:"Сохранить",color:"primary"},on:{click:t.submit}})],1),c("div",{staticClass:"control"},[c("bulma-button",{attrs:{text:"Закрыть"},on:{click:t.close}})],1),c("div",{staticClass:"control"},[c("bulma-button",{attrs:{text:"Удалить",color:"danger",onclick:"return confirm('Вы действительно хотите удалить эту запись?');"}})],1)])],1)},i=[],n=c("e653"),o=c("2bd6"),l=c("e04f"),s=c("ce02"),r=c("7234"),u={id:0,company_id:0,company:r["a"],kind_id:0,kind:r["a"],date_of_practice:"",topic:"",note:""},p=c("0036"),m=c("2af4"),d={name:"PracticeItem",components:{"bulma-button":n["a"],"bulma-date":o["a"],"bulma-input":l["a"],"bulma-select":s["a"]},mixins:[m["a"],p["a"]],data:function(){return{title:"",practice:u,companys:[r["a"]],kinds:[r["a"]]}},mounted:function(){this.fetchItem("practice","Practice",[],["company","kind"],[])},methods:{onSelect:function(t,e){this.practice[e]=t,this.practice["".concat(e,"_id")]=t.id},submit:function(){this.submitItem("practice","Practice",[],[])},delete:function(){}}},b=d,f=(c("0bec"),c("2877")),k=Object(f["a"])(b,a,i,!1,null,"29f30280",null);e["default"]=k.exports}}]);
//# sourceMappingURL=PracticeItem-legacy.3f1a50be.js.map