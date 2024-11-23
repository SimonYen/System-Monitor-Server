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

    //切换主题
    $('#toggle-theme').click(function () {
        var isDark=false;
        if($('#toggle-theme').text()==='🌙'){
            isDark=true;
        }
        cpu_usage_line.dispose();
        if (isDark){
            cpu_usage_line = echarts.init(document.getElementById('cpu-usage-line'),'dark',{height:500,weight:null});
        }else{
            cpu_usage_line = echarts.init(document.getElementById('cpu-usage-line'),null,{height:500,weight:null});
        }
        cpu_usage_line.setOption(option);
    });

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
            if (timeData.length > 99) {
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
    // 每隔 1 秒更新一次图表
    setInterval(updateCPUChart, 1000);
});

//内存部分
$(document).ready(function (){
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

    //切换主题
    $('#toggle-theme').click(function () {
        var isDark=false;
        if($('#toggle-theme').text()==='🌙'){
            isDark=true;
        }
        mem_pie.dispose();
        if (isDark){
            mem_pie = echarts.init(document.getElementById('mem-pie'),'dark',{height:500,weight:null});
        }else{
            mem_pie = echarts.init(document.getElementById('mem-pie'),null,{height:500,weight:null});
        }
        mem_pie.setOption(option);
    });

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


//网络部分
$(document).ready(function () {
    var down_chart = echarts.init(document.getElementById('network-download-speed'),null,{height:500,weight:null});
    var up_chart = echarts.init(document.getElementById('network-upload-speed'),null,{height:500,weight:null});
    var option1 = {
        title: {
            text: 'Network Download Speed Monitor',
            left: 'center'
        },
        tooltip: {
            formatter: '{a} <br/>{b} : {c}'
        },
        series: [
            {
                name: 'Download Speed',
                type: 'gauge',
                progress: {
                    show: true
                },
                detail: {
                    valueAnimation: true,
                    formatter: '{value}'
                },
                data: [
                    {
                        value: 0,
                        name: 'MB'
                    }
                ]
            }
        ]
    };
    var option2 = {
        title: {
            text: 'Network Upload Speed Monitor',
            left: 'center'
        },
        tooltip: {
            formatter: '{a} <br/>{b} : {c}'
        },
        series: [
            {
                name: 'Upload Speed',
                type: 'gauge',
                progress: {
                    show: true
                },
                detail: {
                    valueAnimation: true,
                    formatter: '{value}'
                },
                data: [
                    {
                        value: 0,
                        name: 'MB'
                    }
                ]
            }
        ]
    };
    down_chart.setOption(option1);
    up_chart.setOption(option2);

    //切换主题
    $('#toggle-theme').click(function () {
        var isDark=false;
        if($('#toggle-theme').text()==='🌙'){
            isDark=true;
        }
        down_chart.dispose();
        up_chart.dispose();
        if (isDark){
            up_chart = echarts.init(document.getElementById('network-upload-speed'),'dark',{height:500,weight:null});
            down_chart = echarts.init(document.getElementById('network-download-speed'),'dark',{height:500,weight:null});
        }else{
            up_chart = echarts.init(document.getElementById('network-upload-speed'),null,{height:500,weight:null});
            down_chart = echarts.init(document.getElementById('network-download-speed'),null,{height:500,weight:null});
        }
        down_chart.setOption(option1);
        up_chart.setOption(option2);
    });

    function updateChart(){
        $.getJSON('/api/net',function (data){
            //计算当前网速
            var down_speed=0;
            var up_speed=0;
            data.interfaces.forEach((item)=>{
               down_speed+=item.data.received;
               up_speed+=item.data.transmitted;
            });
            down_chart.setOption({
                series: [{
                    data: [
                        {
                            value: (down_speed/125000).toFixed(2),
                            name: 'Mb'
                        }
                    ]
                }]
            });
            up_chart.setOption({
                series: [{
                    data: [
                        {
                            value: (up_speed/125000).toFixed(2),
                            name: 'Mb'
                        }
                    ]
                }]
            });
        });
    }
    //监测页面大小发生变化时
    // 绑定 resize 事件
    $(window).resize(function () {
        //获取高宽
        var w=$('#network-download-speed').width();
        var h=$('#network-download-speed').height();
        down_chart.resize({height: h,weight: w});
        up_chart.resize({height: h,weight: w});
    });
    setInterval(updateChart, 500);
});