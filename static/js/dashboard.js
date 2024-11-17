//CPU部分
$(document).ready(function () {
    //获取cpu使用率图
    var cpu_usage_line = echarts.init(document.getElementById('cpu-usage-line'),null,{height:500,weight:null});

    var option = {
        title: {
            text: 'CPU Usage Monitor',
            left: 'center'
        },
        tooltip: {},
        xAxis: {
            type: 'category',
            boundaryGap: false,
            data: []
        },
        yAxis: {
            type: 'value',
            axisLabel: {
                formatter: '{value}%' // 添加百分号
            }
        },
        series: [{
            name: 'CPU Usage',
            type: 'line',
            smooth: true,
            areaStyle: {},
            data: []
        }]
    };

    cpu_usage_line.setOption(option);

    // 用于存储时间和 CPU 负载数据
    var timeData = [];
    var cpuData = [];

    // 定义更新图表的函数
    function updateCPUChart() {
        $.getJSON('/api/cpu', function (data) {
            var usageArray = data.usage;
            var now = new Date().toLocaleTimeString();
            // 计算平均值
            var totalUsage=usageArray.reduce((acc,val)=>acc+val,0);
            var averageUsage=totalUsage/usageArray.length;
            // 更新数据
            timeData.push(now);
            cpuData.push(averageUsage);

            // 限制数据点数量
            if (timeData.length > 30) {
                timeData.shift();
                cpuData.shift();
            }
            // 更新 ECharts 数据
            cpu_usage_line.setOption({
                xAxis: {
                    data: timeData
                },
                series: [{
                    data: cpuData
                }]
            });
        });
    }
    //监测页面大小发生变化时
    // 绑定 resize 事件
    $(window).resize(function () {
        //获取高宽
        var w=$('#cpu-usage-line').width();
        var h=$('#cpu-usage-line').height();
        cpu_usage_line.resize({height: h,weight: w});
    });
    // 每隔 2 秒更新一次图表
    setInterval(updateCPUChart, 2000);
});

//内存部分
$(document).ready(function (){
    //获取cpu使用率图
    var mem_pie = echarts.init(document.getElementById('mem-pie'),null,{height:500,weight:null});
    var option = {
        title: {
            text: 'Memory Usage Monitor',
            subtext: 'Unit: GB',
            left: 'center'
        },
        tooltip: {
            trigger: 'item'
        },
        legend: {
            left: 'center',
            orient: 'horizontal', // 水平布局
            bottom: '0%',         // 图例放置在底部
        },
        series: [
            {
                name: 'Memory',
                type: 'pie',
                radius: ['40%', '70%'],
                avoidLabelOverlap: false,
                itemStyle: {
                    borderRadius: 10,
                    borderColor: '#fff',
                    borderWidth: 2
                },
                label: {
                    show: false,
                    position: 'center'
                },
                emphasis: {
                    label: {
                        show: true,
                        fontSize: 40,
                        fontWeight: 'bold'
                    }
                },
                labelLine: {
                    show: false
                },
                data: [
                    { value: 0, name: 'Physical Memory Used' },
                    { value: 0, name: 'Physical Memory Free' },
                    { value: 0, name: 'Swap Memory Used' },
                    { value: 0, name: 'Swap Memory Free' }
                ]
            }
        ]
    };
    mem_pie.setOption(option);
    //定义更新内存文字的函数
    function updateChart(){
        $.getJSON('/api/mem',function (data){
            var free_phy=data.physical_memory.free;
            var used_phy=data.physical_memory.used;
            var free_swap=data.swap_memory.free;
            var used_swap=data.swap_memory.used;
            var d=[
                { value: (used_phy/1024/1024/1024).toFixed(2), name: 'Physical Memory Used' },
                { value: (free_phy/1024/1024/1024).toFixed(2), name: 'Physical Memory Free' },
                { value: (used_swap/1024/1024/1024).toFixed(2), name: 'Swap Memory Used' },
                { value: (free_swap/1024/1024/1024).toFixed(2), name: 'Swap Memory Free' }
            ];
            mem_pie.setOption({
                series: [{
                    data: d
                }]
            });
        });
    }
    //监测页面大小发生变化时
    // 绑定 resize 事件
    $(window).resize(function () {
        //获取高宽
        var w=$('#mem-pie').width();
        var h=$('#mem-pie').height();
        mem_pie.resize({height: h,weight: w});
    });
    setInterval(updateChart, 2000);
});