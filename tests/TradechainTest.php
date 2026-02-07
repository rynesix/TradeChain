<?php
/**
 * Tests for TradeChain
 */

use PHPUnit\Framework\TestCase;
use Tradechain\Tradechain;

class TradechainTest extends TestCase {
    private Tradechain $instance;

    protected function setUp(): void {
        $this->instance = new Tradechain(['verbose' => false]);
    }

    public function testCanCreateInstance(): void {
        $this->assertInstanceOf(Tradechain::class, $this->instance);
    }

    public function testExecuteReturnsSuccess(): void {
        $result = $this->instance->execute();
        $this->assertTrue($result['success']);
        $this->assertArrayHasKey('message', $result);
    }
}
