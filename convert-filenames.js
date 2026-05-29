const fs = require('fs')
const path = require('path')

// 简单的中文转拼音映射（常用字）
const pinyinMap = {
  '猪': 'pig', '小': 'small', '大': 'big', '吃': 'eat', '喝': 'drink',
  '睡': 'sleep', '看': 'look', '哭': 'cry', '笑': 'smile', '跑': 'run',
  '飞': 'fly', '走': 'walk', '坐': 'sit', '站': 'stand', '跳': 'jump',
  '打': 'hit', '杀': 'kill', '死': 'dead', '活': 'live', '爱': 'love',
  '恨': 'hate', '好': 'good', '坏': 'bad', '新': 'new', '旧': 'old',
  '红': 'red', '蓝': 'blue', '绿': 'green', '黄': 'yellow', '白': 'white',
  '黑': 'black', '金': 'gold', '银': 'silver', '天': 'sky', '地': 'ground',
  '人': 'person', '牛': 'cow', '马': 'horse', '羊': 'sheep', '鸡': 'chicken',
  '鱼': 'fish', '鸟': 'bird', '猫': 'cat', '狗': 'dog', '兔': 'rabbit',
  '龙': 'dragon', '蛇': 'snake', '鼠': 'mouse', '星': 'star', '月': 'moon',
  '日': 'sun', '水': 'water', '火': 'fire', '土': 'earth', '木': 'wood',
  '风': 'wind', '雨': 'rain', '雪': 'snow', '云': 'cloud', '花': 'flower',
  '树': 'tree', '草': 'grass', '山': 'mountain', '河': 'river', '海': 'sea',
  '上': 'up', '下': 'down', '左': 'left', '右': 'right', '前': 'front',
  '后': 'back', '中': 'middle', '东': 'east', '西': 'west', '南': 'south',
  '北': 'north', '一': 'one', '二': 'two', '三': 'three', '四': 'four',
  '五': 'five', '六': 'six', '七': 'seven', '八': 'eight', '九': 'nine',
  '十': 'ten', '百': 'hundred', '千': 'thousand', '万': 'ten-thousand',
  '的': 'of', '了': 'le', '在': 'at', '是': 'is', '我': 'i', '你': 'you',
  '他': 'he', '她': 'she', '它': 'it', '们': 'plural', '这': 'this',
  '那': 'that', '有': 'have', '没': 'not', '不': 'no', '和': 'and',
  '与': 'with', '对': 'to', '把': 'take', '被': 'by', '让': 'let',
  '从': 'from', '到': 'to', '过': 'pass', '来': 'come', '去': 'go',
  '出': 'out', '入': 'in', '开': 'open', '关': 'close', '买': 'buy',
  '卖': 'sell', '做': 'do', '说': 'say', '想': 'think', '知': 'know',
  '学': 'learn', '教': 'teach', '写': 'write', '读': 'read', '画': 'draw',
  '唱': 'sing', '跳': 'dance', '听': 'listen', '问': 'ask', '答': 'answer',
  '找': 'find', '等': 'wait', '叫': 'call', '哭': 'cry', '笑': 'laugh',
  '怕': 'afraid', '急': 'urgent', '快': 'fast', '慢': 'slow', '早': 'early',
  '晚': 'late', '冷': 'cold', '热': 'hot', '甜': 'sweet', '苦': 'bitter',
  '酸': 'sour', '辣': 'spicy', '咸': 'salty', '香': 'fragrant', '臭': 'smelly',
  '亮': 'bright', '暗': 'dark', '软': 'soft', '硬': 'hard', '干': 'dry',
  '湿': 'wet', '胖': 'fat', '瘦': 'thin', '高': 'tall', '矮': 'short',
  '长': 'long', '短': 'short', '圆': 'round', '方': 'square', '扁': 'flat',
  '美': 'beautiful', '丑': 'ugly', '真': 'true', '假': 'false', '老': 'old',
  '少': 'young', '男': 'male', '女': 'female', '春': 'spring', '夏': 'summer',
  '秋': 'autumn', '冬': 'winter', '年': 'year', '月': 'month', '日': 'day',
  '时': 'hour', '分': 'minute', '秒': 'second', '今': 'today', '明': 'tomorrow',
  '昨': 'yesterday', '现': 'now', '以': 'after', '会': 'can', '能': 'able',
  '要': 'want', '可': 'may', '该': 'should', '必': 'must', '已': 'already',
  '正': 'just', '才': 'only', '就': 'then', '都': 'all', '也': 'also',
  '还': 'still', '再': 'again', '又': 'again', '很': 'very', '最': 'most',
  '更': 'more', '比': 'than', '太': 'too', '挺': 'quite', '真': 'really',
  '特': 'special', '超': 'super', '极': 'extremely', '死': 'die', '活': 'alive',
  '生': 'life', '病': 'sick', '医': 'doctor', '药': 'medicine', '针': 'needle',
  '刀': 'knife', '枪': 'gun', '炮': 'cannon', '车': 'car', '船': 'boat',
  '机': 'machine', '电': 'electric', '灯': 'lamp', '门': 'door', '窗': 'window',
  '桌': 'table', '椅': 'chair', '床': 'bed', '柜': 'cabinet', '箱': 'box',
  '碗': 'bowl', '杯': 'cup', '盘': 'plate', '筷': 'chopsticks', '勺': 'spoon',
  '刀': 'knife', '叉': 'fork', '纸': 'paper', '笔': 'pen', '书': 'book',
  '本': 'notebook', '包': 'bag', '袋': 'pocket', '钱': 'money', '币': 'coin',
  '卡': 'card', '票': 'ticket', '信': 'letter', '报': 'newspaper', '图': 'picture',
  '画': 'painting', '像': 'image', '照': 'photo', '影': 'shadow', '光': 'light',
  '影': 'shadow', '声': 'sound', '音': 'music', '歌': 'song', '舞': 'dance',
  '戏': 'game', '球': 'ball', '棋': 'chess', '牌': 'card', '琴': 'piano',
  '鼓': 'drum', '钟': 'bell', '铃': 'bell', '旗': 'flag', '绳': 'rope',
  '线': 'thread', '针': 'needle', '钉': 'nail', '锤': 'hammer', '锯': 'saw',
  '剪': 'scissors', '尺': 'ruler', '镜': 'mirror', '锁': 'lock', '钥匙': 'key',
  '药': 'medicine', '毒': 'poison', '糖': 'candy', '盐': 'salt', '醋': 'vinegar',
  '油': 'oil', '酱': 'sauce', '茶': 'tea', '酒': 'wine', '烟': 'smoke',
  '肉': 'meat', '菜': 'vegetable', '饭': 'rice', '面': 'noodle', '汤': 'soup',
  '蛋': 'egg', '奶': 'milk', '果': 'fruit', '瓜': 'melon', '豆': 'bean',
  '米': 'rice', '麦': 'wheat', '玉米': 'corn', '土豆': 'potato', '番茄': 'tomato',
  '洋葱': 'onion', '大蒜': 'garlic', '辣椒': 'pepper', '黄瓜': 'cucumber',
  '西瓜': 'watermelon', '苹果': 'apple', '香蕉': 'banana', '橘子': 'orange',
  '葡萄': 'grape', '草莓': 'strawberry', '桃子': 'peach', '梨': 'pear',
  '芒果': 'mango', '菠萝': 'pineapple', '樱桃': 'cherry', '柠檬': 'lemon',
  '椰子': 'coconut', '核桃': 'walnut', '花生': 'peanut', '芝麻': 'sesame',
  '巧克力': 'chocolate', '蛋糕': 'cake', '面包': 'bread', '饼干': 'biscuit',
  '冰淇淋': 'ice cream', '咖啡': 'coffee', '可乐': 'cola', '果汁': 'juice',
  '啤酒': 'beer', '红酒': 'wine', '白酒': 'liquor', '鸡尾酒': 'cocktail',
  '早餐': 'breakfast', '午餐': 'lunch', '晚餐': 'dinner', '宵夜': 'midnight snack',
  '零食': 'snack', '小吃': 'snack', '快餐': 'fast food', '大餐': 'feast',
  '火锅': 'hotpot', '烧烤': 'bbq', '寿司': 'sushi', '拉面': 'ramen',
  '饺子': 'dumpling', '包子': 'steamed bun', '馒头': 'steamed bread',
  '油条': 'fried dough', '豆浆': 'soy milk', '豆腐': 'tofu', '粽子': 'zongzi',
  '月饼': 'mooncake', '汤圆': 'tangyuan', '春卷': 'spring roll', '炒饭': 'fried rice',
  '炒面': 'chow mein', '炒菜': 'stir fry', '凉菜': 'cold dish', '热菜': 'hot dish',
  '汤': 'soup', '粥': 'porridge', '饭': 'rice', '面条': 'noodles',
  '米粉': 'rice noodles', '粉丝': 'glass noodles', '年糕': 'rice cake',
  '麻花': 'twisted dough', '油条': 'fried dough stick', '烧饼': 'sesame cake',
  '煎饼': 'pancake', '鸡蛋': 'egg', '鸭蛋': 'duck egg', '鹅蛋': 'goose egg',
  '鹌鹑蛋': 'quail egg', '皮蛋': 'preserved egg', '咸蛋': 'salted egg',
  '荷包蛋': 'fried egg', '水煮蛋': 'boiled egg', '蒸蛋': 'steamed egg',
  '炒蛋': 'scrambled eggs', '蛋炒饭': 'egg fried rice', '蛋花汤': 'egg soup',
  '蛋挞': 'egg tart', '蛋糕': 'cake', '奶油': 'cream', '黄油': 'butter',
  '芝士': 'cheese', '酸奶': 'yogurt', '牛奶': 'milk', '羊奶': 'goat milk',
  '豆浆': 'soy milk', '椰汁': 'coconut juice', '果汁': 'juice', '可乐': 'cola',
  '雪碧': 'sprite', '芬达': 'fanta', '美年达': 'mirinda', '七喜': '7up',
  '红牛': 'red bull', '咖啡': 'coffee', '奶茶': 'milk tea', '绿茶': 'green tea',
  '红茶': 'black tea', '乌龙茶': 'oolong tea', '普洱茶': 'puer tea',
  '铁观音': 'tieguanyin', '龙井': 'longjing', '碧螺春': 'biluochun',
  '茉莉花茶': 'jasmine tea', '菊花茶': 'chrysanthemum tea', '柠檬茶': 'lemon tea',
  '蜂蜜': 'honey', '糖': 'sugar', '盐': 'salt', '味精': 'msg', '鸡精': 'chicken essence',
  '酱油': 'soy sauce', '醋': 'vinegar', '料酒': 'cooking wine', '蚝油': 'oyster sauce',
  '番茄酱': 'ketchup', '沙拉酱': 'salad dressing', '芥末': 'mustard', '辣椒酱': 'chili sauce',
  '甜面酱': 'sweet bean sauce', '芝麻酱': 'sesame paste', '花生酱': 'peanut butter',
  '巧克力酱': 'chocolate sauce', '果酱': 'jam', '蜂蜜': 'honey', '枫糖浆': 'maple syrup',
  '炼乳': 'condensed milk', '淡奶油': 'whipping cream', '奶酪': 'cheese', '黄油': 'butter',
  '芝士': 'cheese', '酸奶': 'yogurt', '牛奶': 'milk', '羊奶': 'goat milk',
  '豆浆': 'soy milk', '椰汁': 'coconut juice', '果汁': 'juice', '可乐': 'cola',
  '雪碧': 'sprite', '芬达': 'fanta', '美年达': 'mirinda', '七喜': '7up',
  '红牛': 'red bull', '咖啡': 'coffee', '奶茶': 'milk tea', '绿茶': 'green tea',
  '红茶': 'black tea', '乌龙茶': 'oolong tea', '普洱茶': 'puer tea',
  '铁观音': 'tieguanyin', '龙井': 'longjing', '碧螺春': 'biluochun',
  '茉莉花茶': 'jasmine tea', '菊花茶': 'chrysanthemum tea', '柠檬茶': 'lemon tea',
}

// 将中文转换为拼音
function chineseToPinyin(text) {
  let result = ''
  let i = 0
  while (i < text.length) {
    const char = text[i]
    // 检查是否是中文字符
    if (/[一-龥]/.test(char)) {
      // 尝试匹配多字词
      let matched = false
      for (let len = 4; len >= 2; len--) {
        const word = text.substring(i, i + len)
        if (pinyinMap[word]) {
          result += pinyinMap[word]
          i += len
          matched = true
          break
        }
      }
      if (!matched) {
        // 单字匹配
        result += pinyinMap[char] || char
        i++
      }
    } else if (/[a-zA-Z0-9]/.test(char)) {
      result += char
      i++
    } else if (char === ' ' || char === '-' || char === '_') {
      result += '-'
      i++
    } else {
      // 跳过其他字符（标点符号等）
      i++
    }
  }
  return result
}

// 生成安全的文件名
function generateSafeFilename(originalFilename, index) {
  const ext = path.extname(originalFilename)
  const nameWithoutExt = path.basename(originalFilename, ext)

  // 转换为拼音
  let safeName = chineseToPinyin(nameWithoutExt)

  // 清理：只保留字母、数字、连字符
  safeName = safeName
    .toLowerCase()
    .replace(/[^a-z0-9-]/g, '-')
    .replace(/-+/g, '-')
    .replace(/^-|-$/g, '')

  // 如果名字太短或为空，使用序号
  if (safeName.length < 3) {
    safeName = `pig-${index}`
  }

  // 限制长度
  if (safeName.length > 50) {
    safeName = safeName.substring(0, 50).replace(/-$/, '')
  }

  return `${safeName}${ext}`
}

// 主转换函数
function convertImageList() {
  const dataPath = path.join(__dirname, 'vercel-pig-api', 'data', 'image-list.json')
  const imagesDir = path.join(__dirname, 'images')

  // 读取原始数据
  const data = JSON.parse(fs.readFileSync(dataPath, 'utf8'))
  const images = data.images

  console.log(`开始转换 ${images.length} 张图片...`)

  // 生成新的图片列表
  const newImages = images.map((img, index) => {
    const newFilename = generateSafeFilename(img.filename, index + 1)
    return {
      filename: newFilename,
      title: img.title, // 保留中文标题
      originalFilename: img.filename // 保留原始文件名用于重命名
    }
  })

  // 检查是否有重复的文件名
  const filenameMap = new Map()
  newImages.forEach((img, index) => {
    if (filenameMap.has(img.filename)) {
      // 添加序号避免重复
      const ext = path.extname(img.filename)
      const nameWithoutExt = path.basename(img.filename, ext)
      img.filename = `${nameWithoutExt}-${index + 1}${ext}`
    }
    filenameMap.set(img.filename, index)
  })

  // 重命名实际文件
  let renamedCount = 0
  let errorCount = 0

  newImages.forEach((img) => {
    const oldPath = path.join(imagesDir, img.originalFilename)
    const newPath = path.join(imagesDir, img.filename)

    if (fs.existsSync(oldPath)) {
      try {
        fs.renameSync(oldPath, newPath)
        renamedCount++
      } catch (err) {
        console.error(`重命名失败: ${img.originalFilename} -> ${img.filename}`, err.message)
        errorCount++
      }
    } else {
      console.warn(`文件不存在: ${img.originalFilename}`)
      errorCount++
    }
  })

  // 生成新的 JSON（不包含 originalFilename）
  const newJson = {
    total: data.total,
    images: newImages.map(img => ({
      filename: img.filename,
      title: img.title
    }))
  }

  // 备份原文件
  const backupPath = dataPath + '.backup'
  fs.copyFileSync(dataPath, backupPath)
  console.log(`已备份原文件到: ${backupPath}`)

  // 写入新文件
  fs.writeFileSync(dataPath, JSON.stringify(newJson, null, 2), 'utf8')

  console.log(`\n转换完成！`)
  console.log(`- 成功重命名: ${renamedCount} 个文件`)
  console.log(`- 失败: ${errorCount} 个文件`)
  console.log(`- 新的配置文件已保存到: ${dataPath}`)

  // 生成映射文件供参考
  const mappingPath = path.join(__dirname, 'filename-mapping.json')
  const mapping = newImages.map(img => ({
    original: img.originalFilename,
    new: img.filename,
    title: img.title
  }))
  fs.writeFileSync(mappingPath, JSON.stringify(mapping, null, 2), 'utf8')
  console.log(`- 文件名映射已保存到: ${mappingPath}`)
}

// 运行转换
convertImageList()
