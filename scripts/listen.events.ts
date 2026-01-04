import * as anchor from "@coral-xyz/anchor";
import {Heap} from "heap-js";

const maxHeap = new Heap(Heap.maxComparator);


const buyOrderComparator = (a, b) => {
    if (a.price !== b.price) {
        return b.price - a.price; // Higher price has higher priority
    }
    return a.createdAt - b.createdAt; // Older orders have priority if prices match
};
const buyHeap = new Heap(buyOrderComparator);

const sellOrderComparator = (a, b) => {
    if (a.price !== b.price) {
        return a.price - b.price; // Lower price has higher priority
    }
    return a.createdAt - b.createdAt; 
};
const sellHeap = new Heap(sellOrderComparator);

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.orderBook as anchor.Program;

// Anchor converts your Rust snake_case to camelCase
program.addEventListener("OrderCreatedEvent", (event:any, slot) => {
    const order = {
        id: event.id.toNumber(),
        owner: event.owner.toBase58(),
        quantity: event.quantity.toNumber(),
        remaining: event.remaining.toNumber(), // <--- USE THIS for Heap
        buyPrice: event.buyPrice.toNumber(),
        createdAt: event.createdAt.toNumber(),
    };

    if (event.buyPrice > 0) {
        buyHeap.push(order);
    } else {
        sellHeap.push(order);
    }
    
    console.log(`Order ${order.id} added to heap at slot ${slot}`);
});



async function tryMatch() {
    // 1. Enter a loop to match as many orders as possible
    while (buyHeap.size() > 0 && sellHeap.size() > 0) {
        const bestBid = buyHeap.peek(); // Highest Buy Price
        const bestAsk = sellHeap.peek(); // Lowest Sell Price

        // 2. Check if a match is possible (Price-Time Priority)
        if (bestBid.buyPrice >= bestAsk.sellPrice) {
            console.log(`Match Found! Bid: ${bestBid.buyPrice} >= Ask: ${bestAsk.sellPrice}`);

            // 3. Pop the orders to process them
            const buyOrder = buyHeap.pop();
            const sellOrder = sellHeap.pop();

            try {
                // 4. Call your Solana program match instruction
                

                console.log(`Match Transaction Sent: ${tx}`);


                
            } catch (err) {
                console.error("Match failed, returning orders to heap:", err);
                // If the transaction fails, push them back so they aren't lost
                buyHeap.push(buyOrder);
                sellHeap.push(sellOrder);
                break; // Stop matching for this cycle to avoid infinite error loops
            }
        } else {
            // No more matches possible (Highest bid < Lowest ask)
            break; 
        }
    }
}

tryMatch().catch(console.error);