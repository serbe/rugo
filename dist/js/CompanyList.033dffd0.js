(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["CompanyList"],{"2af4":function(e,t,s){"use strict";t["a"]={methods:{checkArray(e){let t=e?e.filter(e=>""!==e):[];return t.push(""),t},stringArray(e){return e.filter(e=>""!==e)},numberArray(e){return e.filter(e=>""!==e).map(e=>parseInt(e,10))},close(){this.$router.back()}}}},"3b41":function(e,t,s){"use strict";s.r(t);var n=function(){var e=this,t=e.$createElement,s=e._self._c||t;return s("div",{staticClass:"container"},[s("bulma-table",{attrs:{name:"company",names:["Наименование","Адрес","Сфера деятельности","Телефон","Факс","Тренировки"],columns:["name","address","scope_name","phones","faxes","practices"],tableData:e.list,tableClasses:"is-narrow is-striped fullwidth",headClasses:["","is-hidden-touch","is-hidden-mobile","w9 nowrap min130","is-hidden-touch w9 nowrap min130","is-hidden-touch is-hidden-desktop-only w9 nowrap min90"],cellTypes:["text","text","text","phones","phones","array"],pagination:"",hyper:"",search:"",adding:""}})],1)},a=[],i=s("a84d"),r=s("c546"),o=s("2af4"),c={name:"CompanyList",components:{"bulma-table":i["a"]},mixins:[o["a"],r["a"]],mounted(){this.fetchData("company/list","CompanyList")},methods:{createList(e){let t=[];return e&&(t=e.map(e=>{const t=[e.name,e.address,e.scope_name];e.phones&&t.push(e.phones.join(" ")),e.faxes&&t.push(e.faxes.join(" ")),e.practices&&t.push(e.practices.join(" "));const s=e;return s.str=t.join(" ").toLowerCase(),s})),t}}},p=c,l=(s("7fdc"),s("2877")),u=Object(l["a"])(p,n,a,!1,null,null,null);t["default"]=u.exports},"52bc":function(e,t,s){},"7fdc":function(e,t,s){"use strict";var n=s("52bc"),a=s.n(n);a.a}}]);
//# sourceMappingURL=CompanyList.033dffd0.js.map