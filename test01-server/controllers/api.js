const request = require('request-promise-native')
let appkey = require('../appkey')()
const APIError = require('../rest').APIError;

module.exports = {
    'GET /api/stock': async (ctx, next) =>{
        ctx.rest({})
    },

    'GET /api/stock/:id': async (ctx, next) => { //该URL通过controller.js被加入到了router里，router里的规则也是“GET /api/stock/:id”。也就是说id是在koa-router里被解析识别的。
        console.log(`id is ${ctx.params.id}`)
            let data = await request('http://web.juhe.cn:8080/finance/stock/hs?gid=' + ctx.params.id + '&key=' + appkey)
            ctx.rest(JSON.parse(data))    
    },

    // 'POST /api/products': async (ctx, next) => {
    //     var p = products.createProduct(ctx.request.body.name, ctx.request.body.manufacturer, parseFloat(ctx.request.body.price));
    //     ctx.rest(p);
    // },

    // 'DELETE /api/products/:id': async (ctx, next) => {
    //     console.log(`delete product ${ctx.params.id}...`);
    //     var p = products.deleteProduct(ctx.params.id);
    //     if (p) {
    //         ctx.rest(p);
    //     } else {
    //         throw new APIError('product:not_found', 'product not found by id.');
    //     }
    // }
};
